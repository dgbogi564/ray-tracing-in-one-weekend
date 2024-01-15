use crate::vec3::{Point3, Vec3};

pub(crate) struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vec3,
}

/// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
impl Ray {
    pub(crate) fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub(crate) fn default() -> Self {
        Self { origin: Point3::default(), direction : Vec3::default() }
    }

    /// P(t) = A + tb.
    /// - P is a 3D position along a line in 3D.
    /// - A is the ray's point of `origin`.
    /// - b is the ray's `direction`.
    /// - `t` is a scalar that when plugged into P, moves the point along the ray.
    /// Positive `t` gets parts in front of A and negative `t`, parts behind A.
    pub(crate) fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}
