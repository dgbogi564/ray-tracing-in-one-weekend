use std::rc::Rc;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone, Copy)]
pub(crate) struct HitRecord {
    pub(crate) p: Option<Point3>,
    pub(crate) normal: Option<Vec3>,
    pub(crate) t: f64,
    pub(crate) front_face: Option<bool>,
}

impl HitRecord {
    pub(crate) fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p: Some(p),
            normal: Some(normal),
            t,
            front_face: Some(front_face),
        }
    }

    pub(crate) fn default() -> HitRecord {
        HitRecord {
            p: None,
            normal: None,
            t: 0.0,
            front_face: None,
        }
    }

    // The second design decision for normals is whether they should always point outward.
    // We have 2 choices:
    // 1. The normal always points in the direction of the center to the intersection point (the normal points outward).
    // 2. The normal always points against the ray.
    //
    // We need to choose one of these two possibilities because we will eventually need to determine which side of the surface that the ray is coming from.
    // This is important for objects that are rendered differently on each side (e.g. two-sided sheet of paper, glass balls).
    //
    // If we decide to always have normals point out, then we will need to determine which side the ray is when we color it.
    // We can figure this out by comparing the ray with teh normal.
    // - If the ray and the normal face in the same direction, the ray is inside the object and vice versa.
    // This can be determined by taking the dot product of the two vectors:
    // - If the dot is positive, the ray is inside the sphere and vice versa.
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = Some(dot(r.direction, outward_normal) < 0.0);
        self.normal = Some(if self.front_face.unwrap() { outward_normal } else { -outward_normal });
    }
}

pub(crate) trait Hittable {
    fn new(center: Point3, radius: f64) -> Self where Self : Sized;
    fn default() -> Self where Self : Sized;
    fn hit(&self, r: &Ray, ray_t : Interval, rec: &mut HitRecord) -> bool;
}

pub(crate) struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub(crate) fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub(crate) fn hit(&self, r: &Ray, ray_t : Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
