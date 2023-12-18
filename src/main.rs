mod vec3;
mod image;
mod ray;
mod camera;
mod sphere;
mod surface_normals_and_multiple_objects;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class
    image::create();
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
    camera::render(camera::ray_color, "out/ray_color.ppm");
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere
    camera::render(sphere::ray_color, "out/sphere.ppm");
    camera::render(surface_normals_and_multiple_objects::ray_color, "out/sphere_normal.ppm");
}
