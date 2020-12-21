use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: *origin,
            direction: *direction
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
