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
use material::*;
use ray::*;
use simple_logger::SimpleLogger;
use sphere::*;
use utils::*;
use vec3::*;

use image::{ImageBuffer, RgbImage};

fn main() {
    SimpleLogger::new().init().unwrap();

    let ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / ratio) as u32;
    let samper_per_pixel = 10;
    let max_depth = 50;

    let materail_ground = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let center = Box::new(Lambertian::new(Color::new(0.7, 0.8, 0.3)));
    //klet center = Box::new(Dielectric::new(1.5));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_left_2 = Box::new(Dielectric::new(1.5));
    //let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Some(center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Some(materail_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        Some(material_left_2),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));

    let camera = Camera::new();
    let get_offset = |value: u32| -> f64 { value as f64 + get_random_number() };

    let pixle_generator = |x, y| -> image::Rgb<u8> {
        let mut color = Color::default();
        for _ in 0..samper_per_pixel {
            let u = get_offset(x) / (image_width - 1) as f64;
            let v = get_offset(image_height - y) / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, max_depth);
        }

        color /= samper_per_pixel as f64;
        color.fix();
        color.into()
    };

    let image: RgbImage = ImageBuffer::from_fn(image_width, image_height, pixle_generator);
    image.save("pic.png").unwrap();
}
