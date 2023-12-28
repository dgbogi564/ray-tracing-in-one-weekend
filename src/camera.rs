use std::{fs, io};
use std::cmp::max;
use std::io::Write;
use std::path::Path;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Color, Point3, unit_vector, Vec3};

pub(crate) struct Camera {
    image_width: i32,
    aspect_ratio: f64,
    samples_per_pixel: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32) -> Self {
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
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub(crate) fn render(&self, world: &dyn Hittable, file_path: &str, anti_aliasing: bool) {
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
                        pixel_color += Self::ray_color(&r, world);
                    }

                    let scale = 1.0 / self.samples_per_pixel as f64;
                    let intensity = Interval::new(0.000, 0.999);

                    let r = intensity.clamp(pixel_color.x * scale);
                    let g = intensity.clamp(pixel_color.y * scale);
                    let b = intensity.clamp(pixel_color.z * scale);

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

                    let pixel_color = Self::ray_color(&r, world);
                    contents.push_str(&format!("{pixel_color}\n"));
                }
            }
        }


        let directory = Path::new(&file_path).parent().unwrap().to_str().unwrap();
        fs::create_dir_all(directory).expect("Unable to create directory");
        fs::write(file_path, contents).expect("Unable to write file");

        print!("\rDone.                 \n");
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal.unwrap() + Color::new(1.0, 1.0, 1.0));
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
}