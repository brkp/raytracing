use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const BLACK: Color = Color { r: 0x00, g: 0x00, b: 0x00 };
pub const WHITE: Color = Color { r: 0xff, g: 0xff, b: 0xff };
pub const   RED: Color = Color { r: 0xff, g: 0x00, b: 0x00 };
pub const GREEN: Color = Color { r: 0x00, g: 0xff, b: 0x00 };
pub const  BLUE: Color = Color { r: 0x00, g: 0x00, b: 0xff };

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: (255.0 * r) as u8,
            g: (255.0 * g) as u8,
            b: (255.0 * b) as u8,
        }
    }

    pub fn rgb(h: u32) -> Self {
        Self {
            r: (h >> 16) as u8,
            g: (h >>  8) as u8,
            b: (h >>  0) as u8
        }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs).clamp(0.0, 255.0) as u8,
            g: (self.g as f64 * rhs).clamp(0.0, 255.0) as u8,
            b: (self.b as f64 * rhs).clamp(0.0, 255.0) as u8,
        }
    }
}
