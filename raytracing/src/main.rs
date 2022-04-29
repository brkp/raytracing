use cgmath::vec::Vec3;
use canvas::{color, Canvas};

fn main() {
    let vec = Vec3::new(1.0, 1.0, 1.0);
    let mut canvas = Canvas::new(500, 500);

    println!("{:#?}", vec);

    for i in -canvas.hw..=canvas.hw {
        canvas.set_pixel(i, 0, color::WHITE);
        canvas.set_pixel(0, i, color::WHITE);
    }

    canvas.save("main.bmp").unwrap();
}
