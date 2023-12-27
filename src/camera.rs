use std::{fs, io};
use std::cmp::max;
use std::io::Write;
use std::path::Path;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, unit_vector, Vec3};

pub(crate) struct Camera {
    image_width: i32,
    aspect_ratio: f64,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new(image_width: i32, aspect_ratio: f64) -> Self {
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

        Camera { image_width, aspect_ratio, image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }

    pub(crate) fn render(&self, world: &dyn Hittable, file_path: &str) {
        let mut contents = String::with_capacity(2_000_000);

        contents.push_str("P3\n");
        contents.push_str(&format!("{} {}\n", self.image_width, self.image_height));
        contents.push_str("255\n");

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            io::stdout().flush().unwrap();
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);
                contents.push_str(&format!("{pixel_color}\n"));
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
}