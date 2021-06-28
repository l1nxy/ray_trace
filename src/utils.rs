use super::hittable_list::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;
use super::color::*;
use rand::Rng;

pub fn cal_color(ordi: f64) -> u8 {
    (ordi * 255.99) as u8
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

pub fn ray_color(ray: &Ray, world: &HittableList) -> image::Rgb<u8> {
    let mut rec = HitRecord::new();
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
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

