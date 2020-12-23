use std::option::Option;

use crate::hittable::{Hittable,HitRecord,face_normal};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let dir = ray.direction();
        let a = dir.length_squared();
        let half_b = oc.dot(&dir);
        let c = oc.length_squared() - self.radius.powi(2);
        let discrim = half_b.powi(2) - a * c;

        if discrim < 0.0 {
            None
        } else {
            let sqrtd = discrim.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            root = if root < t_min || root > t_max {
                let next_root = (-half_b + sqrtd) / a;
                if next_root < t_min || next_root > t_max {
                    return None
                } else {
                    next_root
                }
            } else {
                root
            };

            let point = ray.point_at(root);
            let outward_normal = (point - self.center) / self.radius;
            let (front_face, n) = face_normal(ray, &outward_normal);
            HitRecord::some_hit(&point, &n, root, front_face)
        }
    }
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Self {
        Sphere {
            center: *center,
            radius: radius
        }
    }
}
