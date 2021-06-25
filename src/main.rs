pub mod ray;
use ray::vec3::*;
use ray::Ray;

use image::{ImageBuffer, RgbImage};
fn cal_color(ordi: f64) -> u8 {
    (ordi * 255.99) as u8
}

fn ray_color(ray: &Ray) -> image::Rgb<u8> {
    let unit_ray = uini_vec3(&ray.dir);
    let t = 0.5 * (unit_ray.y + 1.0);
    let t1 = 1.0 - t;
    image::Rgb([
        cal_color(t1 * 1.0 + t * 0.5),
        cal_color(t1 * 1.0 + t * 0.7),
        cal_color(t1 * 1.0 + t * 1.0),
    ])
}

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> bool {
    let oc = ray.orig - center;
    let a = Vec3::dot(ray.dir, ray.dir);
    let b = Vec3::dot(oc, ray.dir) * 2.0;
    let c = Vec3::dot(oc, oc) - radius * radius;
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
