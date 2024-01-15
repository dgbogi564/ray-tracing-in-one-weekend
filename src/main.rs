use std::rc::Rc;
use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};

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
mod material;

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
    let mat : Rc<dyn Material> = Rc::new(material::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Some(Rc::clone(&mat)))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(Rc::clone(&mat)))));
    let mut camera = camera::Camera::new(400, 16.0 / 9.0, 100, 0);
    camera.render(&world, "out/camera.ppm", Camera::ray_color,
                  false, false);

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#antialiasing
    camera.samples_per_pixel = 10;
    camera.render(&world, "out/anti_aliasing.ppm", Camera::ray_color,
                  true, false);

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials
    camera.max_depth = 50;
    camera.render(&world, "out/diffuse.ppm", Camera::ray_color_diffuse,
                  true, false);
    camera.render(&world, "out/lambertian_diffuse.ppm", Camera::ray_color_lambertian_diffuse,
                  true, false);
    camera.render(&world, "out/gamma_diffuse.ppm", Camera::ray_color_lambertian_diffuse,
                  true, true);

    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#metal
    let material_ground: Rc<dyn Material>   = Rc::new(material::Lambertian::new(
        Color::new(0.8, 0.8, 0.0)));
    let material_center : Rc<dyn Material>  = Rc::new(material::Lambertian::new(
        Color::new(0.7, 0.3, 0.3)));
    let material_left : Rc<dyn Material>  = Rc::new(material::Metal::new(
        Color::new(0.8, 0.8, 0.8)));
    let material_right : Rc<dyn Material>  = Rc::new(material::Metal::new(
        Color::new(0.8, 0.6, 0.2)));
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0,
                                  Some(Rc::clone(&material_ground)))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5,
                                  Some(Rc::clone(&material_center)))));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5,
                                  Some(Rc::clone(&material_left)))));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5,
                                  Some(Rc::clone(&material_right)))));
    let cam = Camera::new(400, 16.0 / 9.0, 100, 50);

    cam.render(&world, "out/metal.ppm", Camera::ray_color_lambertian_diffuse, true, true);
}
