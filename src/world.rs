use std::cmp::max;
use std::{fs, io};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, unit_vector, Point3, Vec3};

pub(crate) fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal.unwrap() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(r.direction);
    let a = 0.5*(unit_direction.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.6, 0.7, 1.0)
}

pub(crate) fn render(file_path: &str) {
    let mut contents = String::with_capacity(2_000_000);

    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;

    // Calculate image height, ensuring that it's at least 1.
    let image_height: i32 = max((image_width as f64 / aspect_ratio).floor() as i32, 1);

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    // Distance between the camera center and the viewport where everything is in perfect focus.
    let focal_length = 1.0;


    // The virtual viewport is a representation of what the user/camera sees.
    // We'll use this viewport as a plane through which to pass our scene rays.
    //
    // Viewport widths less than one are ok since they are real valued.
    //
    // In order for the pixels to be spaces teh same distance horizontally as they are vertically,
    // the viewport that bounds them must have teh same aspect ratio as the rendered image.
    //
    // We choose our viewport height arbitrarily.
    //
    // We don't use `aspect_ratio` here because the actual ratio between the image width and height
    // may be different because the image height was floored and clamped to integers.
    //
    // Unlike our image, the viewport can be any real number rather than just an integer.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Point in 3D space from which all scene rays will originate
    // (commonly referred to as the eye point).
    // The camera center will be orthogonal to the viewport center.
    // For simplicity, we'll start the camera center at (0,0,0).
    let camera_center = Point3::new(0.0, 0.0, 0.0);


    // To represent our space, we'll also have the y-axis go up, the x-axis to the right, and the
    // negative z-axis pointing in the viewing direction (this is commonly referred to as right-
    // handed coordinates).
    //
    // However, these conventions conflict with our image coordinate system in which the 0th pixel
    // is the top left and we work our way down the last pixel in the bottom right. In other words,
    // the image coordinate's y-axis is inverted: y increases going down the image.
    //
    // To help navigate the pixel grid, we'll use a vector (Vu) from the left edge to the right edge
    // and a vector (Vv) from the upper edge to the lower edge.

    // Calculates vectors calculate the vectors across the horizontal and down the vertical viewport
    // edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Our pixel grid will be inset from the viewport edges by half the pixel-to-pixel distance
    // (0.5 unit borders). This way our viewport area is evenly divided into width × height
    // identical regions.
    //
    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    contents.push_str("P3\n");
    contents.push_str(&format!("{image_width} {image_height}\n"));
    contents.push_str("255\n");

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            contents.push_str(&format!("{pixel_color}\n"));
        }
    }

    let directory = Path::new(&file_path).parent().unwrap().to_str().unwrap();
    fs::create_dir_all(directory).expect("Unable to create directory");
    fs::write(file_path, contents).expect("Unable to write file");

    print!("\rDone.                 \n");
}
