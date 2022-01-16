use std::ops;
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b
        }
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

