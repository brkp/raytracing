use crate::color::Color;

use std::fs::File;
use std::io::{Write, BufWriter};

pub struct Image {
    pub w: u32,
    pub h: u32,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            w, h,
            data: vec![Color::default(); (w * h) as usize]
        }
    }

    pub fn get_frame(&mut self) -> &mut [Color] {
        &mut self.data
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if (0..self.w).contains(&x) && (0..self.h).contains(&y) {
            self.data[(x + y * self.w) as usize] = color;
        }
    }

    pub fn export(&self, path: &str) -> std::io::Result<()> {
        let mut output = BufWriter::new(File::create(path)?);

        let padding = ((4 - (self.w * 3) % 4) % 4) as u32;
        let (head_size, data_size) = self.calculate_size(padding);

        output.write_all(&self.generate_file_header(head_size, data_size))?;
        output.write_all(&self.generate_info_header(data_size))?;

        for chunk in self.data.chunks(self.w as usize).rev() {
            chunk
                .iter()
                .try_for_each(|c| output.write_all(&[c.b, c.g, c.r]))?;
            output.write_all(&[0, 0, 0][..padding as usize])?;
        }

        Ok(())
    }

    fn calculate_size(&self, padding: u32) -> (u32, u32) {
        let head_size = (14 + 40) as u32;
        let data_size = self.w * self.h * 3 + self.h * padding;

        (head_size, data_size)
    }

    fn generate_file_header(&self, head_size: u32, data_size: u32) -> [u8; 14] {
        let mut file_header = [0u8; 14];

        file_header[0x0..0x2].copy_from_slice(&[0x42, 0x4d]);
        file_header[0x2..0x6].copy_from_slice(&(head_size + data_size).to_le_bytes());
        file_header[0x6..0xa].copy_from_slice(&0u32.to_le_bytes());
        file_header[0xa..0xe].copy_from_slice(&head_size.to_le_bytes());

        file_header
    }

    fn generate_info_header(&self, data_size: u32) -> [u8; 40] {
        let mut data_header = [0u8; 40];

        data_header[0x00..0x04].copy_from_slice(&40u32.to_le_bytes());
        data_header[0x04..0x08].copy_from_slice(&self.w.to_le_bytes());
        data_header[0x08..0x0c].copy_from_slice(&self.h.to_le_bytes());
        data_header[0x0c..0x0e].copy_from_slice(&1u16.to_le_bytes());
        data_header[0x0e..0x10].copy_from_slice(&24u16.to_le_bytes());
        data_header[0x10..0x14].copy_from_slice(&0u32.to_le_bytes());
        data_header[0x14..0x18].copy_from_slice(&data_size.to_le_bytes());
        data_header[0x18..0x28].copy_from_slice(&0u128.to_le_bytes());

        data_header
    }
}
