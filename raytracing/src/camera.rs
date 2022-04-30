use cgmath::Point3;

#[derive(Debug, Default)]
pub struct Camera {
    pub pos: Point3,
}

impl Camera {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { pos: Point3::new(x, y, z) }
    }
}
