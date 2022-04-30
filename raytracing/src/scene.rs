use canvas::{color::Color, Canvas};
use cgmath::Vec3;

use crate::camera::Camera;
use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn Hittable>>,
    pub viewport: Viewport,
}

#[derive(Debug, Default)]
pub struct Viewport {
    pub w: f32,
    pub h: f32,
    pub z: f32,
}

impl Viewport {
    pub fn new(w: f32, h: f32, z: f32) -> Self {
        Self { w, h, z }
    }

    pub fn translate(&self, x: i32, y: i32, canvas: &Canvas) -> Vec3 {
        Vec3::new(
            x as f32 * self.w / canvas.image.w as f32,
            y as f32 * self.h / canvas.image.h as f32,
            self.z,
        )
    }
}

impl Scene {
    pub fn new(cam: Camera, viewport: Viewport) -> Self {
        Self {
            camera: cam,
            shapes: vec![],
            viewport,
        }
    }

    fn trace_ray(&self, ray: &Ray) -> Option<Color> {
        let mut closest_t = f32::INFINITY;
        let mut closest_shape: Option<&Box<dyn Hittable>> = None;

        for shape in &self.shapes {
            if let Some(hit_record) = shape.hit(ray, 0.0, f32::INFINITY) {
                if closest_t > hit_record.t {
                    closest_t = hit_record.t;
                    closest_shape = Some(shape);
                }
            }
        }

        if let Some(shape) = closest_shape {
            return Some(shape.get_color());
        }

        None
    }

    pub fn render(&self, canvas: &mut Canvas) {
        for y in -canvas.hh..=canvas.hh {
            for x in -canvas.hw..=canvas.hw {
                let ray = Ray::new(self.camera.pos, self.viewport.translate(x, y, &canvas));
                if let Some(color) = self.trace_ray(&ray) {
                    canvas.set_pixel(x, y, color)
                }
            }
        }
    }
}
