use ultraviolet::Vec3;

use crate::{ray::Ray, hittable::HitRecord, sphere::Sphere};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub enum MaterialTypes {
    Empty,
    Lambertian(Vec3),
    Metal(Vec3)
}

impl Material for MaterialTypes {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            &MaterialTypes::Lambertian(albedo) => {
                attenuation.x = albedo.x;
                attenuation.y = albedo.y;
                attenuation.z = albedo.z;
                return self.scatter_lambertian(r_in, rec, scattered);
            },
            &MaterialTypes::Metal(albedo) => {
                attenuation.x = albedo.x;
                attenuation.y = albedo.y;
                attenuation.z = albedo.z;
                return self.scatter_metal(r_in, rec, scattered);
            },
            _ => panic!("empty material!")
        }
    }
}

impl MaterialTypes {
    fn scatter_lambertian(&self, _: &Ray, rec: &HitRecord, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Sphere::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        scattered.origin = rec.p;
        scattered.dir = scatter_direction;
        true
    }

    fn scatter_metal(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> bool {
        let reflected = r_in.dir.normalized().reflect(rec.normal);
        scattered.origin = rec.p;
        scattered.dir = reflected;
        scattered.dir.dot(rec.normal) > 0.0
    }
}

trait NearZero {
    fn near_zero(&self) -> bool;
}

impl NearZero for Vec3 {
    fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

trait Reflect {
    fn reflect(&self, n: Vec3) -> Vec3;
}

impl Reflect for Vec3 {
    fn reflect(&self, n: Vec3) -> Vec3 {
        *self - (2.0 * self.dot(n) * n)
    }
}

