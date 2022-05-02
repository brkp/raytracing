use canvas::color::Color;
use cgmath::{Point3, Vec3};

use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};

pub struct Sphere {
    pub rad: f32,
    pub pos: Point3,
}

impl Sphere {
    pub fn new(pos: Point3, rad: f32) -> Self {
        Sphere { rad, pos }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.pos;

        let a: f32 = ray.direction.len2();
        let h: f32 = ray.direction.dot(oc);
        let c: f32 = oc.len2() - self.rad.powf(2.0);
        let d: f32 = h * h - a * c;

        if d < 0.0 {
            return None;
        }

        let d = d.sqrt();
        let mut r = (-h - d) / a;

        if t_min > r || t_max < r {
            r = (-h + d) / a;
            if t_min > r || t_max < r {
                return None;
            }
        }

        let p = ray.at(r);
        let mut h = HitRecord::default();

        h.t = r;
        h.p = p;
        h.set_face_normal(ray, &((p - self.pos) / self.rad));

        return Some(h);
    }
}
