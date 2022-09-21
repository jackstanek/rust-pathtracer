use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color,Point};

pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitRecord, attenuation: &Color) -> Option<Ray>;
}

pub struct Lambertian {
    color: Color
}

impl Lambertian {

}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hr: &HitRecord, attenuation: &Color) -> Option<Ray> {
        let target = hr.point + hr.normal + Point::new_rand_in_sphere().unit_vector();
        Some(Ray::new(&hr.point, &(target - hr.point)))
    }
}
