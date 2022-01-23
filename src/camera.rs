use ultraviolet::Vec3;
use super::Ray;
use rand::Rng;

pub struct Camera {
    pub origin: Vec3,
    pub top_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32, // vertical fov in degrees
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (lookat - lookfrom).normalized();
        let u = w.cross(vup);
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let top_left_corner = origin - horizontal/2.0 - vertical/2.0 + focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            top_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let rd = self.lens_radius * Camera::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let dir = self.top_left_corner + x * self.horizontal + y * self.vertical - self.origin - offset;
        Ray::new(self.origin + offset, dir)
    }

    fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.mag_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}
