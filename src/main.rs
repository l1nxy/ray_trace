mod camera;
mod color;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;
use camera::*;
use color::*;
use hittable_list::*;
use log::Log;
use material::*;
use ray::*;
use simple_logger::SimpleLogger;
use sphere::*;
use utils::*;
use vec3::*;

use image::{ImageBuffer, RgbImage};
fn random_scene() -> Box<HittableList> {
    let mut world = Box::new(HittableList::new());

    let materail_ground = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Some(materail_ground),
    )));

    for i in -11..11 {
        for j in -11..11 {
            let random_mat = get_random_number();
            let center = Vec3::new(
                i as f64 + 0.9 * get_random_number(),
                0.2,
                j as f64 + 0.9 * get_random_number(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if random_mat < 0.8 {
                    let albedo = Color::random();
                    let mat = Box::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, Some(mat))));
                } else if random_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 0.9);
                    let fuzz = get_random_number_range(0.0, 0.5);
                    let mat = Box::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, Some(mat))));
                } else {
                    let mat = Box::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, Some(mat))));
                }
            }
        }
    }

    let mat1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Some(mat1),
    )));

    let mat2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(mat2),
    )));

    let mat3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Some(mat3),
    )));
    world
}
fn main() {
    SimpleLogger::new().init().unwrap();
    log::warn!("start!");
    let ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / ratio) as u32;
    let samper_per_pixel = 1;
    let max_depth = 10;

    let world = random_scene();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, ratio, 0.1, dist_to_focus);
    let get_offset = |value: u32| -> f64 { value as f64 + get_random_number() };

    let pixle_generator = |x, y| -> image::Rgb<u8> {
        let mut color = Color::default();
        for _ in 0..samper_per_pixel {
            let u = get_offset(x) / (image_width - 1) as f64;
            let v = get_offset(y) / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, max_depth);
        }

        color /= samper_per_pixel as f64;
        color.fix();
        color.into()
    };

    let image: RgbImage = ImageBuffer::from_fn(image_width, image_height, pixle_generator);
    image.save("pic.png").unwrap();
    log::warn!("end!");
}
