mod vec3;
mod image;
mod ray;
mod camera;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class
    image::create();
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
    camera::render();
}
