use std::ops::{Add, AddAssign, DivAssign, Mul};
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(ir: f64, ig: f64, ib: f64) -> Self {
        Self {
            r: ir,
            g: ig,
            b: ib,
        }
    }

    pub fn fix(&mut self) {
        let f = |x: f64| -> f64 {
            let mut ret = x.sqrt();
            if ret < 0.0 {
                ret = 0.0;
            }
            if ret > 1.0 {
                ret = 1.0;
            }
            ret
        };

        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
    }
}

impl From<image::Rgb<u8>> for Color {
    fn from(value: image::Rgb<u8>) -> Self {
        Self {
            r: (value.0[0] as f64 / 256.0),
            g: (value.0[1] as f64 / 256.0),
            b: (value.0[2] as f64 / 256.0),
        }
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(value: Color) -> Self {
        Self([
            (value.r * 255.99) as u8,
            (value.g * 255.99) as u8,
            (value.b * 255.99) as u8,
        ])
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

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
