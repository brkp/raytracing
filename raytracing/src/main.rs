mod vec;

use canvas::{color, Canvas};

fn main() {
    let mut canvas = Canvas::new(500, 500);

    for i in -canvas.hw..=canvas.hw {
        canvas.set_pixel(i, 0, color::WHITE);
        canvas.set_pixel(0, i, color::WHITE);
    }

    canvas.save("main.bmp").unwrap();
}
