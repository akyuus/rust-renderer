use std::rc::Rc;
use ultraviolet::Vec3;
use crate::material::MaterialTypes;

use super::{Ray, material::Material};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_rc: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // ray is outside the sphere, don't change the normal (facing the ray)
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal }
    }

    pub fn new() -> Self {
        HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            mat_rc: Rc::new(MaterialTypes::Empty),
            t: 0.0,
            front_face: false
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<'a> {
    pub objects: Vec<Box<dyn Hittable + 'a>>
}

impl<'a> HittableList<'a> {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add<T: Hittable + 'a>(&mut self, obj: T) {
        self.objects.push(Box::new(obj))
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            mat_rc: Rc::new(MaterialTypes::Empty),
            t: t_max,
            front_face: false
        };

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if (*obj).hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // i guess copying is fine here? need a more idiomatic way to do this
                *rec = HitRecord {
                    p: temp_rec.p,
                    normal: temp_rec.normal,
                    mat_rc: Rc::clone(&temp_rec.mat_rc),
                    t: temp_rec.t,
                    front_face: temp_rec.front_face
                }
            }
        }
        hit_anything
    }
}
