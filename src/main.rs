use std::rc::Rc;
use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

mod vec3;
mod image;
mod ray;
mod old_camera;
mod hit_sphere;
mod surface_normals_and_multiple_objects;
mod hittable;
mod sphere;
mod utils;
mod world;
mod interval;
mod camera;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class
    image::create();

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
    old_camera::render(old_camera::ray_color, "out/ray_color.ppm");

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere
    old_camera::render(hit_sphere::ray_color, "out/sphere.ppm");

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#surfacenormalsandmultipleobjects
    old_camera::render(surface_normals_and_multiple_objects::ray_color, "out/sphere_normal.ppm");
    world::render("out/world.ppm");

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#movingcameracodeintoitsownclass
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let mut camera = camera::Camera::new(400, 16.0 / 9.0, 100, 0);
    camera.render(&world, "out/camera.ppm", false, Camera::ray_color);

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#antialiasing
    camera.samples_per_pixel = 10;
    camera.render(&world, "out/anti_aliasing.ppm", true, Camera::ray_color);

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials
    camera.max_depth = 50;
    camera.render(&world, "out/diffuse.ppm", true, Camera::ray_color_diffuse);
}
