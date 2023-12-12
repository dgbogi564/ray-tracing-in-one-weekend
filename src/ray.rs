use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

/// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// P(t) = A + tb.
    ///
    /// * P is a 3D position along a line in 3D.
    /// * A is the ray `origin`.
    /// * b is the ray `direction`.
    /// * `t` is a real number that when plugged into P, moves the point along the ray.
    /// * * Positive `t` gets parts in front of A and negative `t`, parts behind A.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}