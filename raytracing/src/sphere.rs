use canvas::color::Color;
use cgmath::{Point3, Vec3};

use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};

pub struct Sphere {
    rad: f32,
    pos: Point3,
    color: Color,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        todo!()
    }
}
