use rand::Rng;
use ultraviolet::Vec3;

use crate::{ray::Ray, hittable::HitRecord, sphere::Sphere};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub enum MaterialTypes {
    Empty,
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32)
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
            &MaterialTypes::Metal(albedo, mut fuzz) => {
                attenuation.x = albedo.x;
                attenuation.y = albedo.y;
                attenuation.z = albedo.z;
                // gotta clamp
                fuzz = fuzz.clamp(0.0, 1.0);
                return self.scatter_metal(r_in, rec, scattered, fuzz);
            },
            &MaterialTypes::Dielectric(index_of_refraction) => {
                attenuation.x = 1.0;
                attenuation.y = 1.0;
                attenuation.z = 1.0;
                return self.scatter_dielectric(r_in, rec, scattered, index_of_refraction);
            }
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

    fn scatter_metal(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray, fuzz: f32) -> bool {
        let reflected = r_in.dir.normalized().reflect(rec.normal);
        scattered.origin = rec.p;
        scattered.dir = reflected + fuzz * Sphere::random_in_unit_sphere();
        scattered.dir.dot(rec.normal) > 0.0
    }

    fn scatter_dielectric(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray, ir: f32) -> bool {
        let refraction_ratio = if rec.front_face { 1.0/ir } else { ir };
        let unit_dir = r_in.dir.normalized();
        let cos_theta = (-1.0 * unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // refraction is impossible in this case
        if refraction_ratio * sin_theta > 1.0 || MaterialTypes::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0) {
            scattered.origin = rec.p;
            scattered.dir = unit_dir.reflect(rec.normal);
        }
        else {
            scattered.origin = rec.p;
            scattered.dir = unit_dir.refract(rec.normal, refraction_ratio);
        }
        return true;
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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

trait Refract {
    fn refract(&self, n: Vec3, n1_over_n2: f32) -> Vec3;
}

impl Refract for Vec3 {
    fn refract(&self, n: Vec3, n1_over_n2: f32) -> Vec3 {
        let cos_theta = (-1.0 * *self).dot(n).min(1.0);
        let out_perp = n1_over_n2 * (*self + cos_theta*n);
        let out_parallel = -((1.0 - out_perp.mag_sq()).abs().sqrt()) * n;
        out_perp + out_parallel
    }
}

