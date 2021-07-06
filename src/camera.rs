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
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        //let ratio = 16.0 / 9.0;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(u, w).unit();

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let left_of_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
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
