use crate::bmp::Image;
use crate::color::Color;

pub struct Canvas {
    pub hw: i32,
    pub hh: i32,
    pub image: Image,
}

impl Canvas {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            hw: (w / 2) as i32,
            hh: (h / 2) as i32,
            image: Image::new(w, h),
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.image.get_frame().iter_mut().for_each(|c| *c = color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.image.set_pixel(
            (self.hw + x) as u32,
            (self.hh - y) as u32,
            color,
        );
    }

    pub fn export(&self, path: &str) -> std::io::Result<()> {
        self.image.export(path)
    }
}
