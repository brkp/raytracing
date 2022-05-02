use crate::ray::Ray;

use canvas::color::Color;
use cgmath::{Point3, Vec3};

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3,    // intersection point
    pub n: Vec3,      // surface normal
    pub f: bool,      // front face or not
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.f = ray.direction.dot(*outward_normal) < 0.0;
        self.n = if self.f { *outward_normal } else { *outward_normal * -1.0 };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
