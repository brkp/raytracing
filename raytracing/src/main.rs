#![allow(warnings)]

mod camera;
mod hit;
mod ray;
mod scene;
mod sphere;

use canvas::{color, Canvas};
use cgmath::{Point3, Vec3};

use crate::camera::Camera;
use crate::hit::Hittable;
use crate::scene::{Scene, Viewport};

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let canvas_width: u32 = 400;
    let canvas_height = (canvas_width as f32 / aspect_ratio) as u32;

    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let mut mscene = scene::Scene::new(
        Camera::default(),
        Viewport::new(2.0 * aspect_ratio, 2.0, -1.0),
    );

    mscene.shapes.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        color::RED,
    )));

    canvas.clear(color::Color::rgb(0x6495ed));
    mscene.render(&mut canvas);
    canvas.export("main.bmp").unwrap();
}
