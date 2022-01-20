use std::ops;
use ultraviolet::Vec3;
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub trait IntoU8 {
    fn into_u8(self) -> u8;
}

impl IntoU8 for f32 {
    fn into_u8(self) -> u8 {
        (self.clamp(0.0, 1.0) * 255.0).floor() as u8
    }
}

impl IntoU8 for u8 {
    fn into_u8(self) -> u8 {
        self
    }
}

impl Color {
    pub fn new<T>(r: T, g: T, b: T) -> Color 
    where T: IntoU8 
    {
        Color {
            r: r.into_u8(),
            g: g.into_u8(),
            b: b.into_u8()
        }
    }

    pub fn average_samples(v: Vec3, samples_per_pixel: i32) -> Color {
        let scale = 1.0 / samples_per_pixel as f32;
        
        // gamma correction
        let scaled_v = Vec3::new(
            (v.x * scale).sqrt(),
            (v.y * scale).sqrt(),
            (v.z * scale).sqrt()
        );

        Color::from(scaled_v)
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        fn to_u8(f: f32) -> u8 { 
            f.ceil().clamp(0.0, 255.0) as u8
        }

        Color {
            r: to_u8(self.r as f32 * rhs),
            g: to_u8(self.g as f32 * rhs),
            b: to_u8(self.b as f32 * rhs)
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {

        fn safe_add(x1: u8, x2: u8) -> u8 {
            match x1.checked_add(x2) {
                Some(x3) => x3,
                None => 255
            }
        }

        Color {
            r: safe_add(self.r, rhs.r),
            g: safe_add(self.g, rhs.g),
            b: safe_add(self.b, rhs.b)
        }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color {
            r: (256.0 * v.x).clamp(0.0, 255.0) as u8,
            g: (256.0 * v.y).clamp(0.0, 255.0) as u8,
            b: (256.0 * v.z).clamp(0.0, 255.0) as u8
        }
    }
}
