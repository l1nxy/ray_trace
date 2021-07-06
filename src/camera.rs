use super::ray::*;
use super::vec3::*;
use std::f64::consts::*;

pub struct Camera {
    pub orig: Vec3,
    pub lower_left_corner: Vec3,
    pub hori: Vec3,
    pub vert: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * PI / 180.0;
        let ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let left_of_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            orig: origin,
            lower_left_corner: left_of_corner,
            hori: horizontal,
            vert: vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let dir = self.lower_left_corner + self.hori * u + self.vert * v - self.orig;
        Ray::new(&self.orig, &dir)
    }
}
