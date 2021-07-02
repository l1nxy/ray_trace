use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::utils::get_random_number_range;
use crate::utils::get_random_number;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn uini_vec3(vec: &Vec3) -> Vec3 {
    let l = vec.length();
    Vec3 {
        x: vec.x / l,
        y: vec.y / l,
        z: vec.z / l,
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_unit() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        let ret = self.length_squared();
        ret.sqrt()
    }

    pub fn unit(&self) -> Self {
        uini_vec3(self)
    }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.y * rhs.z - rhs.y * lhs.z,
            y: 0.0 - (lhs.x * rhs.z - rhs.x * lhs.z),
            z: lhs.x * rhs.y - rhs.x * lhs.y,
        }
    }

    pub fn random() -> Self {
        Self {
            x: get_random_number(),
            y: get_random_number(),
            z: get_random_number(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: get_random_number_range(min, max),
            y: get_random_number_range(min, max),
            z: get_random_number_range(min, max),
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < 0.00000000001 && self.y.abs() < 0.00000000001 && self.z.abs() < 0.00000000001
    }

    pub fn reflect(self, rhs: Vec3) -> Self {
        self - rhs * 2.0 * Vec3::dot(self, rhs)
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        self + (0.0 - rhs)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
