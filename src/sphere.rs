use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64, mat: Option<Rc<dyn Material>>) -> Sphere {
        Sphere { center, radius, mat }
    }
}

impl Hittable for Sphere {
    /// Most ray tracers have found it convenient to add a valid interval for hits tmin to tmax,
    /// so the hit only “counts” if tmin < t < tmax.
    /// Initial rays will be positive t, but can be simplified later on.
    ///
    /// Design question: should be compute the normals if we hit something?
    /// No, we may hit something closer to the origin, so we don't have to render it.
    /// Regardless, we'll start with a simple solution.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let delta = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(delta, r.direction);
        let c = delta.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = Some(r.at(root));
        let outward_normal = (rec.p.unwrap() - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        if self.mat.is_none() {
            rec.mat = None
        } else {
            rec.mat = Some(Rc::clone(&self.mat.as_ref().unwrap()));
        }

        true
    }
}
