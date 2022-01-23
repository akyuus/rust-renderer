use std::rc::Rc;
use rand::Rng;
use ultraviolet::Vec3;
use crate::{material::MaterialTypes, sphere::Sphere};

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

    pub fn new() -> HittableList<'a> {
        HittableList { 
            objects: vec![] 
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add<T: Hittable + 'a>(&mut self, obj: T) {
        self.objects.push(Box::new(obj))
    }

    pub fn random_scene() -> HittableList<'a> {
        let mut world = HittableList::new();
        let mut rng = rand::thread_rng();
        let ground_material: Rc<dyn Material> = Rc::new(MaterialTypes::Lambertian(Vec3::new(0.5, 0.5, 0.5)));
        world.add(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            mat_rc: Rc::clone(&ground_material)
        });

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = rng.gen_range(-1.0..1.0);
                let center = Vec3::new(a as f32 + 0.9 * choose_mat, -0.2, b as f32 + 0.9 * choose_mat);

                if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                    let mut sphere_material = MaterialTypes::Empty;

                    if choose_mat < 0.8 {
                        // diffuse 
                        let albedo = HittableList::random_vec3(0.0, 1.0) * HittableList::random_vec3(0.0, 1.0);
                        sphere_material = MaterialTypes::Lambertian(albedo);
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            mat_rc: Rc::new(sphere_material)
                        });
                    }
                    else if choose_mat < 0.95 {
                        // metal
                        let albedo = HittableList::random_vec3(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        sphere_material = MaterialTypes::Metal(albedo, fuzz);
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            mat_rc: Rc::new(sphere_material)
                        });
                    }
                    else {
                        // glass
                        sphere_material = MaterialTypes::Dielectric(1.5);
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            mat_rc: Rc::new(sphere_material)
                        });
                    }
                }
            }
        }

        let material1: Rc<dyn Material> = Rc::new(MaterialTypes::Dielectric(1.5));
        world.add(Sphere {
            center: Vec3::new(0.0, -1.0, 0.0),
            radius: 1.0,
            mat_rc: Rc::clone(&material1)
        });

        let material2: Rc<dyn Material> = Rc::new(MaterialTypes::Lambertian(Vec3::new(0.4, 0.2, 0.1)));
        world.add(Sphere {
            center: Vec3::new(-4.0, -1.0, 0.0),
            radius: 1.0,
            mat_rc: Rc::clone(&material2)
        });

        let material3: Rc<dyn Material> = Rc::new(MaterialTypes::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0));
        world.add(Sphere {
            center: Vec3::new(4.0, -1.0, 0.0),
            radius: 1.0,
            mat_rc: Rc::clone(&material3)
        });

        return world;
    }

    fn random_vec3(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        let (rand_x, rand_y, rand_z) = (rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
        Vec3::new(rand_x, rand_y, rand_z)
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
