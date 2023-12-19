use crate::ray::Ray;
use crate::vec3::{Color, dot, norm, Point3};

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere
// The formula for the radius of sphere at the origin of a 3D space is x^2 + y^2 + z^2 = r^2.
// You can use this formula to determine which points are within or outside of the sphere.
// E.g. for a given point (x, y, z), if x^2 + y^2 + z^2 < r^2, its inside the sphere and vice versa.
//
// However, in order to apply this formula to a sphere centered at any arbitrary center point,
// the formula becomes (x - C_x)^2, (y - C_y)^2 + (z - C_z)^2 = r^2, where C is the center point.
// In other words, if P = (x, y, z) and C = (C_x, C_y, C_z), the formula is (P - C) ⋅ (P - C) = r^2.
//
// We can also read this equation as "any point P that satisfies this equation is on the sphere";
// a satisfying P indicates that the point is apart of the sphere's surface.
// To find if a ray P(t) = A + tb  ever hits the sphere, we find if there exists some t for which
// P(t) satisfies the sphere equation:
//     (P(t) - C) ⋅ (P(t) - C) = r^2
// or it's expanded form:
//     ((A + tb) - C) ⋅ ((A + tb) - C) = r^2
// We want to solve for t, so we'll separate the terms based on whether there is a t or not:
//     (tb + (A - C)) ⋅ (tb + (A - C)) = r^2
// and then distribute the dot product:
//     t^2b ⋅ b + 2tb ⋅ (A - C) + (A - C) ⋅ (A - C) = r^2
// finally, move the square of the radius over the left hand to find 0:
//     t^2b ⋅ b + 2tb ⋅ (A - C) + (A - C) ⋅ (A - C) - r^2 = 0
// Taking into account that the only unknown in this equation is t, the formula becomes an quadratic
// equation which we can solve with the following formula:
//     (-b ± sqrt(b^2 - 4ac))/2a
// in which:
//     a = b ⋅ b
//     b = 2b ⋅ (A - C)
//     c = (A - C) ⋅ (A - C) - r^2
// thus, we can solve for t.
//
// In the formula, b can either be positive or negative in the formula which correlates with the
// geometry of the sphere:
// - the ray does not intersect with the sphere (0 roots)
// - the ray is tangent to the sphere (1 root, b = 0)
// - the ray intersects through the sphere (2 roots, enter/exit)

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let delta = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2.0 * dot(delta, r.direction);
    let c = dot(delta, delta) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

pub(crate) fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = norm(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.6, 0.7, 1.0)
}