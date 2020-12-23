use std::option::Option;

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64, /* point along a ray originating at the camera */
    pub front_face: bool
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    /* Helper function to construct a successful hit */
    pub fn some_hit(point: &Vec3, normal: &Vec3, t: f64, front_face: bool) -> Option<Self> {
        Some(HitRecord {
            point: *point,
            normal: *normal,
            t: t,
            front_face: front_face
        })
    }
}

/* Normals should always point "outward", i.e. against the incident ray */
pub fn face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
    let front_face = ray.direction().dot(outward_normal) < 0.0;
    let n = if front_face {
        *outward_normal
    } else {
        -(*outward_normal)
    };
    (front_face, n)
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) -> () {
        self.objects.push(object)
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for obj in &self.objects {
            let latest_hit = obj.hit(ray, t_min, t_max);
            match (latest_hit, closest_hit) {
                (Some(new_hit), Some(old_hit)) => closest_hit = if new_hit.t < old_hit.t {
                    latest_hit
                } else {
                    closest_hit
                },
                (Some(_), None) => closest_hit = latest_hit,
                (None, _) => ()
            }
        }
        closest_hit
    }
}
