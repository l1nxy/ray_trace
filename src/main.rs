#![deny(clippy::all)]
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
use rayon::prelude::*;
use sphere::*;
use utils::*;
use vec3::*;

use std::sync::Arc;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let materail_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, -0.0),
        radius: 1000.0,
        mat: materail_ground,
    }));

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
                    let mat = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat,
                    }));
                } else if random_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 0.9);
                    let fuzz = get_random_number_range(0.0, 0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat,
                    }));
                } else {
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat,
                    }));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: mat1,
    }));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: mat2,
    }));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: mat3,
    }));
    world
}
fn main() {
    let image_width: usize = 600;
    let image_height: usize = 400;
    let samper_per_pixel = 10;
    let max_depth = 50;

    let world = random_scene();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, 3.0 / 2.0, 0.1, dist_to_focus);

    let mut pixel = Vec::with_capacity(image_width * image_height);
    for j in (0..image_height).rev() {
        let par_iter = (0..image_width).into_par_iter().map(|i| {
            let mut color = (0..samper_per_pixel)
                .into_iter()
                .fold(Color::default(), |acc, _| {
                    let (x, y) = (i as f64, j as f64);
                    let u = (x + get_random_number()) / (image_width - 1) as f64;
                    let v = (y + get_random_number()) / (image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    acc + ray_color(&ray, &world, max_depth)
                })
                / samper_per_pixel as f64;
            color.fix();
        });

        let mut line_pixel: Vec<Color> = par_iter.collect();
        pixel.append(&mut line_pixel);
    }
    let buffer =
        image::ImageBuffer::from_vec(image_width as u32, image_height as u32, pixel).unwrap();
    buffer.save("pic.png").unwrap();
}
