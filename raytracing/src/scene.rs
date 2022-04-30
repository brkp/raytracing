use canvas::Canvas;
use cgmath::Vec3;

use crate::camera::Camera;
use crate::hit::{HitRecord, Hittable};

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
}
