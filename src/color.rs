use std::ops::{Add, AddAssign, DivAssign};
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl Color {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn fix(&mut self) {
        let f = |x: i32| -> i32 {
            let mut ret = x;
            if ret < 0 {
                ret = 0;
            }
            if ret > 255 {
                ret = 255;
            }
            ret
        };

        let g = |x: i32| -> i32 {
            let _r = (x as f64 / 256.0).sqrt();
            (_r * 256.0) as i32
        };
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);

        self.r = g(self.r);
        self.g = g(self.g);
        self.b = g(self.b);
    }
}

impl From<image::Rgb<u8>> for Color {
    fn from(value: image::Rgb<u8>) -> Self {
        Self {
            r: value.0[0] as i32,
            g: value.0[1] as i32,
            b: value.0[2] as i32,
        }
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(value: Color) -> Self {
        Self([value.r as u8, value.g as u8, value.b as u8])
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl DivAssign<i32> for Color {
    fn div_assign(&mut self, rhs: i32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
