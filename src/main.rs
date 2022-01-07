use std::f32::{MAX, INFINITY};
use image::{RgbImage, ImageBuffer};
use nalgebra::{Vector3};

const VIEWPORT_WIDTH: i32 = 1;
const VIEWPORT_HEIGHT: i32 = 1;
const CANVAS_WIDTH: i32 = 200;
const CANVAS_HEIGHT: i32 = 200;


struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    color: (u8, u8, u8)
}
fn main() {
    let mut img: RgbImage = ImageBuffer::new(CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32);
    let o = Vector3::new(0.0,0.0,0.0);
    let scene_spheres = vec![
        Sphere {
            center: Vector3::new(0.0, 1.0, 3.0),
            radius: 1.0,
            color: (255,0,0)
        },
        Sphere {
            center: Vector3::new(-2.0, 2.0, 4.0),
            radius: 1.0,
            color: (0,0,255)
        },
        Sphere {
            center: Vector3::new(2.0, 2.0, 4.0),
            radius: 1.0,
            color: (0,255,0)
        }
    ];
    for x in (-CANVAS_WIDTH / 2)..CANVAS_WIDTH/2 {
        for y in (-CANVAS_HEIGHT / 2)..CANVAS_HEIGHT/2 {
            let cv = Vector3::new(x, y, 1);
            let d = canvas_to_viewport(cv);
            let color = trace_ray(o, d, 1.0, MAX, &scene_spheres);
            let (x1, y1) = change_coordinates(x, y);
            img.put_pixel(x1 as u32, y1 as u32, image::Rgb([color.0, color.1, color.2]));
        }
    }

    img.save("output.tga").unwrap();
}

fn change_coordinates(x: i32, y: i32) -> (i32, i32) {
    (CANVAS_WIDTH / 2 + x, CANVAS_HEIGHT / 2 + y)
}

fn trace_ray(start: Vector3<f32>, dir: Vector3<f32>, t_min: f32, t_max: f32, scene_spheres: &Vec<Sphere>) -> (u8, u8, u8) {
    let mut closest_t = INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;

    for sphere in scene_spheres {
        let (t1, t2) = intersect_ray_sphere(start, dir, sphere);
        if t1 > t_min && t1 < t_max {
            closest_t = t1;
            closest_sphere = Some(&sphere);
        }
        if t2 > t_min && t2 < t_max {
            closest_t = t2;
            closest_sphere = Some(&sphere);
        }

    }

    match closest_sphere {
        None => return (255,255,255),
        Some(s) => return s.color
    }
}

fn intersect_ray_sphere(start: Vector3<f32>, dir: Vector3<f32>, sphere: &Sphere) -> (f32, f32) {
    let r = sphere.radius;
    let co = start - sphere.center;

    let a = dir.dot(&dir);
    let b = 2.0 * co.dot(&dir);
    let c = co.dot(&co) - r*r;

    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return (MAX, MAX);
    }

    let t1 = -b + disc.sqrt();
    let t2 = -b - disc.sqrt();
    (t1, t2)
}

fn canvas_to_viewport(cv: Vector3<i32>) -> Vector3<f32> {
    let ratio_h = (VIEWPORT_HEIGHT as f32) / CANVAS_HEIGHT as f32;
    let ratio_w = (VIEWPORT_WIDTH as f32) / CANVAS_WIDTH as f32;
    let v= Vector3::new(cv.x as f32 * ratio_w, cv.y as f32 * ratio_h, 1.0);
    v
}