use std::rc::Rc;

use ultraviolet::Vec3;
use crate::material::Material;

use super::hittable::{Hittable, HitRecord};
use super::Ray;
use rand::Rng;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat_rc: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat_rc: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_rc
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let (rand_x, rand_y, rand_z) = (rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            let v = Vec3::new(rand_x, rand_y, rand_z);
            if v.mag_sq() >= 1.0 {
                continue;
            }
            return v;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Sphere::random_in_unit_sphere().normalized()
    }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let co = r.origin - self.center;
        let a = r.dir.mag_sq();
        let half_b = r.dir.dot(co);
        let c = co.mag_sq() - self.radius * self.radius;
        let discriminant = half_b*half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a; 
        if root < t_min || t_max < root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_rc = Rc::clone(&self.mat_rc);
        true
    }
}

