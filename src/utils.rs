use super::color::*;
use super::hittable_list::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;
use std::rc::Rc;

pub fn ray_color(ray_in: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::new(Rc::new(None));
    if depth <= 0 {
        return Color::default();
    }
    if world.hit(*ray_in, 0.0001, f64::INFINITY, &mut rec) {
        // let target = rec.p + rec.normal + uini_vec3(&random_in_unit_sphere());
        // let new_ray_dir = target - rec.p;
        // let new_ray = Ray::new(&rec.p, &new_ray_dir);
        // return ray_color(&new_ray, world, depth - 1) * 0.5;

        let mut scattered = Ray::new(&Vec3::default(), &Vec3::default());
        let mut color = Color::default();
        let mat = rec.mat.clone();
        if let Some(x) = &(*mat) {
            if (*x).scatter(ray_in, &mut rec, &mut color, &mut scattered) {
                return ray_color(&scattered, world, depth - 1) * color;
            }
            return Color::new(0.0,0.0,0.0);
        }
    }

    let unit_ray = uini_vec3(&ray_in.dir);
    let t = 0.5 * (unit_ray.y + 1.0);
    let t1 = 1.0 - t;
    Color::new(t1 * 1.0 + t * 0.5, t1 * 1.0 + t * 0.7, t1 * 1.0 + t * 1.0)
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
