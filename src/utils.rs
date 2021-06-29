use super::color::*;
use super::hittable_list::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

pub fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Color::default();
    }
    if world.hit(*ray, 0.000000000000001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + uini_vec3(&random_in_unit_sphere());
        let new_ray_dir = target - rec.p;
        let new_ray = Ray::new(&rec.p, &new_ray_dir);
        return ray_color(&new_ray, world, depth - 1) * 0.5;
    }

    let unit_ray = uini_vec3(&ray.dir);
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
