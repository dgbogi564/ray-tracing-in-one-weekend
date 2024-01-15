use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, random_unit_vector, reflect, unit_vector, Vec3};

// Material needs to do two things:
//    1. Produce a scattered ray (or say it absorbed the incident ray).
//    2. If it scattered, say how much the ray should be attenuated.
pub(crate) trait Material {
    fn new(a: Color) -> Self where Self: Sized;
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)
               -> bool;
}

pub(crate) struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn new(a: Color) -> Self {
        Self { albedo: a }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let normal = rec.normal.unwrap();
        let mut scatter_direction: Vec3 = normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        *scattered = Ray::new(rec.p.unwrap(), scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub(crate) struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn new(a: Color) -> Self {
        Self { albedo: a }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction), rec.normal.unwrap());
        *scattered = Ray::new(rec.p.unwrap(), reflected);
        *attenuation = self.albedo;
        true
    }
}