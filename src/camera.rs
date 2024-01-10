use std::{fs, io};
use std::cmp::max;
use std::io::Write;
use std::path::Path;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Color, Point3, random_on_hemisphere, random_unit_vector, unit_vector, Vec3};

pub(crate) struct Camera {
    image_width: i32,
    aspect_ratio: f64,
    pub(crate) samples_per_pixel: i32,
    pub(crate) max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32, max_depth: i32)
                      -> Self {
        let image_height: i32 = max((image_width as f64 / aspect_ratio).floor() as i32, 1);

        let focal_length = 1.0;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let center = Point3::new(0.0, 0.0, 0.0);

        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub(crate) fn render(&self, world: &dyn Hittable, file_path: &str,
                         ray_color: fn(&Ray, &dyn Hittable, i32) -> Color,
                         anti_aliasing: bool, gamma_correction: bool) {
        let mut contents = String::with_capacity(2_000_000);

        contents.push_str("P3\n");
        contents.push_str(&format!("{} {}\n", self.image_width, self.image_height));
        contents.push_str("255\n");

        // When a real camera takes a picture, there are usually no jagged edges, because edge pixels
        // are a blend of some foreground and background. Unlike rendered images, a true image of the
        // world has infinite resolution and is continuous. We can get a similar effect by averaging
        // a bunch of samples for each pixel.
        //
        // With a single ray intersecting through the center of each pixel, we are performing point
        // sampling.
        //
        // If a checkerboard consisted of an 8x8 grid of black and white tiles, but only four rays
        // hit it, then all four rays may intersect with only black, white, or some combination of
        // both. However, in the real world, when we perceive a checkerboard from far away it looks gray
        // instead of sharp points of black or white.
        //
        // In other words, our eyes integrate (the continuous function of) light falling on a
        // particular (discrete) region of our rendered image.
        //
        // We don't want to integrate the center of the pixel multiple times, but instead the light
        // falling around the pixel, and then integrate those samples to approximate a true
        // continuous result.
        //
        // So how do we integrate the light falling around the pixel?
        // One way is via gaussian blur: sample the light falling around the pixel and average their
        // values together.
        if anti_aliasing {
            for j in 0..self.image_height {
                print!("\rScanlines remaining: {} ", self.image_height - j);
                io::stdout().flush().unwrap();
                for i in 0..self.image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j);
                        pixel_color += ray_color(&r, world, self.max_depth);
                    }

                    let scale = 1.0 / self.samples_per_pixel as f64;
                    let intensity = Interval::new(0.000, 0.999);

                    let mut r = pixel_color.x * scale;
                    let mut g = pixel_color.y * scale;
                    let mut b = pixel_color.z * scale;

                    if gamma_correction {
                        r = Camera::linear_to_gamma(r);
                        g = Camera::linear_to_gamma(g);
                        b = Camera::linear_to_gamma(b);
                    }

                    let r = intensity.clamp(r);
                    let g = intensity.clamp(g);
                    let b = intensity.clamp(b);

                    contents.push_str(&format!("{0}\n", Color::new(r, g, b)));
                }
            }
        } else {
            for j in 0..self.image_height {
                print!("\rScanlines remaining: {} ", self.image_height - j);
                io::stdout().flush().unwrap();
                for i in 0..self.image_width {
                    let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                    //
                    let ray_origin = self.center;
                    let ray_direction = pixel_center - ray_origin;
                    let r = Ray::new(ray_origin, ray_direction);

                    let pixel_color = Self::ray_color(&r, world, self.max_depth);
                    contents.push_str(&format!("{pixel_color}\n"));
                }
            }
        }


        let directory = Path::new(&file_path).parent().unwrap().to_str().unwrap();
        fs::create_dir_all(directory).expect("Unable to create directory");
        fs::write(file_path, contents).expect("Unable to write file");

        print!("\rDone.                 \n");
    }

    pub(crate) fn ray_color(r: &Ray, world: &dyn Hittable, _max_depth: i32) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal.unwrap() + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.6, 0.7, 1.0)
    }

    // Diffuse objects that don't emit their own light merely take on the color of their
    // surroundings, but they do modulate with their own intrinsic color.
    //
    // Light that reflects off a diffuse surface has its direction randomized, so, if we send
    // three rays into a crack between two diffuse surfaces they will each have different random
    // behavior.
    //
    // They might also be absorbed rather than reflected.
    //
    // An algorithm that randomizes direction will produce surfaces that look matte.
    // The simplest diffuse material is one in which it has an equal chance of reflecting light
    // in any direction.
    pub(crate) fn ray_color_diffuse(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // 0.001 - fix shadow acne: bug associated with floating point rounding errors on object
        // intersections
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = random_on_hemisphere(rec.normal.unwrap());
            return 0.5 * Self::ray_color_diffuse(&Ray::new(rec.p.unwrap(), direction), world,
                                                 depth - 1);
        }

        let unit_direction = unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.6, 0.7, 1.0)
    }

    pub(crate) fn ray_color_lambertian_diffuse(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // 0.001 - fix shadow acne: bug associated with floating point rounding errors on object
        // intersections
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = random_on_hemisphere(rec.normal.unwrap());

            // Lambertian reflection
            // Creates scatters via a random distribution of unit vectors proportional to cos
            // (theta) where theta is the angle between the reflected ray and the surface normal.
            //
            // The intersection point, P, of a ray to the surface of the sphere has exactly two
            // sides: inside and outside the sphere.
            //
            // We can create two unit spheres tangent to tha surface:
            //    one sphere in the direction of the surface's normal (P + n)
            //    and one vice versa (P - n)
            //
            // We want our reflections to reflect in the same direction as the ray's origin, so
            // with our current implementation in mind, we'll only worry about the unit sphere in
            // the direction of the surface normal.
            //
            // We then take a random point, S, on the unit radius of the selected sphere and send
            // a ray originating from the contact point, P, to the generated point, S (this is
            // the vector S - P).
            let direction = rec.normal.unwrap() + random_unit_vector();
            return 0.5 * Self::ray_color_diffuse(&Ray::new(rec.p.unwrap(), direction), world,
                                                 depth - 1);
        }

        let unit_direction = unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.6, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sample camera ray for the pixel at location i, j.

        let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + random_double!();
        let py = -0.5 + random_double!();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        f64::sqrt(linear_component)
    }
}