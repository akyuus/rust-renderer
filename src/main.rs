pub mod ray;
pub mod color;
pub mod hittable;
pub mod sphere;
pub mod camera;
pub mod material;

use std::{rc::Rc};

use hittable::{Hittable, HitRecord};
use image::{RgbImage};
use ultraviolet::Vec3;
use color::Color;
use ray::Ray;
use camera::Camera;
use rand::Rng;

use crate::{hittable::HittableList, sphere::Sphere, material::{MaterialTypes, Material}};

fn main() {

    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: u8 = 20;
    const ASPECT_RATIO: f32 = 3.0/2.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let world = HittableList::random_scene();
    // let material_ground: Rc<dyn Material> = Rc::new(MaterialTypes::Lambertian(Vec3::new(0.8, 0.8, 0.0)));
    // let material_center: Rc<dyn Material> = Rc::new(MaterialTypes::Lambertian(Vec3::new(0.1, 0.2, 0.5)));
    // let material_left: Rc<dyn Material> = Rc::new(MaterialTypes::Dielectric(1.5));
    // let material_right: Rc<dyn Material> = Rc::new(MaterialTypes::Metal(Vec3::new(0.8, 0.6, 0.2), 0.0));

    // let mut world = HittableList::new();
    // world.add(Sphere {
    //     center: Vec3::new(0.0,100.5, 1.0),
    //     radius: 100.0,
    //     mat_rc: Rc::clone(&material_ground)
    // });
    // world.add(Sphere {
    //     center: Vec3::new(0.0,0.0, 1.0),
    //     radius: 0.5,
    //     mat_rc: Rc::clone(&material_center)
    // });
    // world.add(Sphere {
    //     center: Vec3::new(-1.0,0.0, 1.0),
    //     radius: 0.5,
    //     mat_rc: Rc::clone(&material_left)
    // });
    // world.add(Sphere {
    //     center: Vec3::new(1.0,0.0, 1.0),
    //     radius: 0.5,
    //     mat_rc: Rc::clone(&material_right)
    // });
    
    let lookfrom = Vec3::new(13.0, -2.0, -3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, -1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        aperture,
        10.0
    );
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let mut color_vector = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + rng.gen_range(0.0..1.0)) / ((IMAGE_WIDTH - 1) as f32);
                let v = (y as f32 + rng.gen_range(0.0..1.0))/ ((IMAGE_HEIGHT - 1) as f32);
                let r = cam.get_ray(u, v);
                color_vector += ray_color(&r, &world, MAX_DEPTH);
            }
            let averaged_color = Color::average_samples(color_vector, SAMPLES_PER_PIXEL);
            img.put_pixel(x, y, image::Rgb([averaged_color.r, averaged_color.g, averaged_color.b]));
        }
        println!("Done with row {}", x + 1);
    } 

    img.save("output.png").unwrap(); 
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u8) -> Vec3 {

    if depth <= 0 {
        return Vec3::zero();
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();
        let mat_rc = Rc::clone(&rec.mat_rc);
        if mat_rc.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1)
        }
    }

    let mut unit_direction = r.dir;
    unit_direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * t + Vec3::new(0.5, 0.7, 1.0) * (1.0 - t)
}
