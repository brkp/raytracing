use canvas::color::Color;
use cgmath::{Point3, Vec3};

use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};

pub struct Sphere {
    pub rad: f32,
    pub pos: Point3,
    pub color: Color,
}

impl Sphere {
    pub fn new(pos: Point3, rad: f32, color: Color) -> Self {
        Sphere { rad, pos, color }
    }
}

impl Hittable for Sphere {
    fn get_color(&self) -> Color {
        self.color
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.pos;

        let a: f32 = ray.direction.len2();
        let h: f32 = ray.direction.dot(oc);
        let c: f32 = oc.len2() - self.rad.powf(2.0);
        let d: f32 = h * h - a * c;

        if d >= 0.0 {
            let d = d.sqrt();
            let r = (-h + d) / a;

            if (t_min..=t_max).contains(&r) {
                let p = ray.at(r);
                return Some(HitRecord {
                    t: r,
                    p,
                    n: (p - self.pos) / self.rad,
                });
            }
        }

        None
    }
}
