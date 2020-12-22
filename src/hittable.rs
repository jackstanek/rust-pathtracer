use std::option::Option;

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64 /* point along a ray originating at the camera */
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    /* Helper function to construct a successful hit */
    pub fn some_hit(point: &Vec3, normal: &Vec3, t: f64) -> Option<Self> {
        Some(HitRecord {
            point: *point,
            normal: *normal,
            t: t
        })
    }
}
