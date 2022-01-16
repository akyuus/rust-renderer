pub mod ray_tracer;
pub mod color;

use std::f32::{MAX, INFINITY};
use image::{RgbImage, ImageBuffer};
use ultraviolet::Vec3;
use color::Color;
use ray_tracer::Ray;

fn main() {

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let or = Vec3::new(0.0,0.0,0.0);
    let mut img = RgbImage::new(IMG_WIDTH, IMG_HEIGHT);

    fn canvas_to_viewport(x: u32, y: u32) -> Vec3 {
        let shifted_x = x as i32 - (IMG_WIDTH / 2) as i32;
        let shifted_y = y as i32 - (IMG_HEIGHT / 2) as i32;
        
        Vec3::new(
            shifted_x as f32 * VIEWPORT_WIDTH / IMG_WIDTH as f32,
            shifted_y as f32 * VIEWPORT_HEIGHT / IMG_HEIGHT as f32,
            FOCAL_LENGTH
        )
    }

    for x in 0..IMG_WIDTH {
        for y in 0..IMG_HEIGHT {
            let dir = canvas_to_viewport(x, y);
            let r = Ray::new(or, dir);
            let pixel_color = ray_color(&r);
            img.put_pixel(x, y, image::Rgb([pixel_color.r, pixel_color.g, pixel_color.b]));
        }
    }

    img.save("output.png").unwrap();
}

fn ray_color(r: &Ray) -> Color {
    let mut unit_direction = r.dir;
    unit_direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    Color::new(255, 255, 255) * (1.0 - t) + Color::new(255/2, 178, 26) * t
}
