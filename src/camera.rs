use ultraviolet::Vec3;
use super::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
    pub img_width: u32,
    pub img_height: u32
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const IMAGE_WIDTH: u32 = 400;

        Camera {
            img_width: IMAGE_WIDTH,
            img_height: (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32,
            viewport_height: VIEWPORT_HEIGHT,
            viewport_width: ASPECT_RATIO * VIEWPORT_HEIGHT,
            focal_length: 1.0,
            origin: Vec3::zero()
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let dir = self.canvas_to_viewport(x, y);
        Ray::new(self.origin, dir)
    }

    fn canvas_to_viewport(&self, x: f32, y: f32) -> Vec3 {
        let shifted_x = x - (self.img_width / 2) as f32;
        let shifted_y = y - (self.img_height / 2) as f32;
        
        Vec3::new(
            shifted_x * self.viewport_width / self.img_width as f32,
            shifted_y * self.viewport_height / self.img_height as f32,
            self.focal_length
        )
    }
}
