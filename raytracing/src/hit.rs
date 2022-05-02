use crate::ray::Ray;

use canvas::color::Color;
use cgmath::{Point3, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3,
    pub n: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
