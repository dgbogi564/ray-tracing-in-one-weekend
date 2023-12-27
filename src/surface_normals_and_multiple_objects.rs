// A surface normal is a vector perpendicular to the tangent plane of a surface.
//
// We require unit length normal vectors in several places. (TODO why?)
//
// If you require normal vectors ot be unit length, then you can often efficiently generate that
// vector with an understanding of the specific geometry class, in its construct, or in the `hit()`
// function.
//
// Can generate unit length vectors efficiently in the hit function of specific geometries.
// e.g. sphere normals can be made unit length simply by dividing by the sphere radius, avoiding
// the square root entirely.
//
// For a sphere, the outward normal is in the direction of the hit point minus the center.
// Origin point is P, direction is the opposite direction that P is to C.
// If earth is the sphere, the earth's center to you points straight up, in the same direction as
// the surface normal.
//
// No light, so we visualize normals with a color map.
// For surface normal we need hit point, not just hit detection.
// Only 1 sphere in the scene, so don't have to worry about negative values of t.
// Assume the closest hit point (smallest t) is the one we want.

use crate::ray::Ray;
use crate::vec3::{Color, dot, unit_vector, Point3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let delta = r.origin - center;
    // before: let a = dot(r.direction, r.direction);
    // vector dotted with itself is equal to the squared length of that vector.
    let a = r.direction.length_squared();

    // before: let b = 2.0 * dot(delta, r.direction);
    // Let h be the dot product such that:
    //     b = 2 * h
    // Since b is a multiplied by 2, the
    //     b^2 = 4h^2
    // If you factor out 4 from b^2 and 4ac, you can simplify the numerator of the equation to:
    //     -2h ± 2 * sqrt(h^2 - ac).
    // The denominator is 2a, so you can further simplify the full equation  to:
    //     -h ± * sqrt(h^2 - ac)/a
    let half_b = dot(delta, r.direction);

    // before: let c = dot(delta, delta) - radius * radius;
    let c = delta.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 { -1.0 } else { -half_b - f64::sqrt(discriminant) / a }
}

pub(crate) fn ray_color(r: &Ray) -> Color {
    let center = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(center, 0.5, r);
    if t > 0.0  {
        let n = unit_vector(r.at(t) - center); // surface normal
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = unit_vector(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.6, 0.7, 1.0)
}
