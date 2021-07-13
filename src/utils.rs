use super::color::*;
use super::hittable_list::*;
use super::ray::*;
use super::vec3::*;

pub fn ray_color(ray_in: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth == 0 {
        return Color::default();
    }
    if let Some(ret) = world.hit(*ray_in, 0.001, f64::MAX) {
        let mut scattered = Ray::new(&Vec3::default(), &Vec3::default());
        let mut color = Color::default();
        if ret.mat.scatter(ray_in, &ret, &mut color, &mut scattered) {
            ray_color(&scattered, world, depth - 1) * color
        } else {
            Color::default()
        }
    } else {
        let unit_ray = uini_vec3(&ray_in.dir);
        let t = 0.5 * (unit_ray.y + 1.0);
        let t1 = 1.0 - t;
        Color::new(t1 * 1.0 + t * 0.5, t1 * 1.0 + t * 0.7, t1 * 1.0 + t * 1.0)
    }
}

pub fn get_random_number() -> f64 {
    rand::random::<f64>()
}

pub fn get_random_number_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            get_random_number_range(-1.0, 1.0),
            get_random_number_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
