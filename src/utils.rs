use super::color::*;
use super::hittable_list::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;
use rand::Rng;

pub fn cal_color(ordi: f64) -> u8 {
    (ordi * 256.0) as u8
}

pub fn cal_color_vec(point: Vec3) -> image::Rgb<u8> {
    image::Rgb([cal_color(point.x), cal_color(point.y), cal_color(point.z)])
}

// pub fn clamp_color(color: Color,min_color:f64,max_color:f64) ->image::Rgb<u8>{
//     let r = 0_u8;
//     let g = 0_u8;
//     let b = 0_u8;
//     let checker = | color : i32| -> u8{
//         if color < min_color {
//             (color * 256)
//         }
//     }
// }

pub fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> image::Rgb<u8> {
    let mut rec = HitRecord::new();
    //if depth <= 0 {
    //let point = Vec3::default();
    //return cal_color_vec(point);
    //}
    //if world.hit(*ray, 0.0, f64::INFINITY, &mut rec) {
    //let target = rec.p + rec.normal + random_in_unit_sphere();
    //let new_ray_dir = target - rec.p;
    //let new_ray = Ray::new(&rec.p, &new_ray_dir);
    //let mut color: Color = ray_color(&new_ray, world, depth - 1).into();
    //color.r /= 2;
    //color.g /= 2;
    //color.b /= 2;
    //let ret: image::Rgb<u8> = color.into();
    //return ret;
    //}

    if world.hit(*ray, 0.0, f64::INFINITY, &mut rec) {
        return cal_color_vec((rec.normal + 1.0) * 0.5);
    }
    let unit_ray = uini_vec3(&ray.dir);
    let t = 0.5 * (unit_ray.y + 1.0);
    let t1 = 1.0 - t;
    image::Rgb([
        cal_color(t1 * 1.0 + t * 0.5),
        cal_color(t1 * 1.0 + t * 0.7),
        cal_color(t1 * 1.0 + t * 1.0),
    ])
}

pub fn get_random_number() -> f64 {
    rand::random::<f64>()
}

pub fn get_random_number_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
