pub mod ray;
mod sphere;
use ray::vec3::*;
use ray::Ray;

use image::{ImageBuffer, RgbImage};
fn cal_color(ordi: f64) -> u8 {
    (ordi * 255.99) as u8
}

fn cal_color_vec(point: Vec3) -> image::Rgb<u8> {
    image::Rgb([cal_color(point.x), cal_color(point.y), cal_color(point.z)])
}
fn ray_color(ray: &Ray) -> image::Rgb<u8> {
    let t_center = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t_center > 0.0 {
        let n = ray.at(t_center) - Vec3::new(0.0, 0.0, -1.0);
        return cal_color_vec((n + 1.0) * 0.7);
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

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.orig - center;
    let a = ray.dir.length_squared();
    let hafl_b = Vec3::dot(oc, ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discirminant = hafl_b.powi(2) - a * c;
    if discirminant < 0.0 {
        -1.0
    } else {
        (-hafl_b - discirminant.sqrt()) / a
    }
}
fn main() {
    let ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (400.0 / ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let left_of_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let image: RgbImage = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let u = x as f64 / (image_width - 1) as f64;
        let v = (image_height - y) as f64 / (image_height - 1) as f64;
        let dir = left_of_corner + horizontal * u + vertical * v - origin;
        let ray = Ray::new(&origin, &dir);
        ray_color(&ray)
    });
    image.save("pic.png").unwrap();
}
