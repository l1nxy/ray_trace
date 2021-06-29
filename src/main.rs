mod camera;
mod color;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;
use camera::*;
use color::*;
use hittable_list::*;
use ray::*;
use sphere::*;
use utils::*;
use vec3::*;
use simple_logger::SimpleLogger;


use image::{ImageBuffer, RgbImage};

fn main() {

    SimpleLogger::new().init().unwrap();

    let ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (400.0 / ratio) as u32;
    let samper_per_pixel = 100;
    let max_depth = 100;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();
    let get_offset = |value: u32| -> f64 { value as f64 + get_random_number() };

    let pixle_generator = |x, y| -> image::Rgb<u8> {
        let mut color = Color::default();
        for _ in 0..samper_per_pixel {
            let u = get_offset(x) / (image_width -1) as f64;
            let v = get_offset(image_height - y) / (image_height -1) as f64;

            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world,max_depth);
        }

        color /= samper_per_pixel as f64;
        color.fix();
        color.into()
    };

    let image: RgbImage = ImageBuffer::from_fn(image_width, image_height, pixle_generator);
    image.save("pic.png").unwrap();
}
