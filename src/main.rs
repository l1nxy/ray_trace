
mod vec3;
use vec3::Vec3;
use image::{ImageBuffer, RgbImage};
fn main() {
    let b = 0.25;
    let image: RgbImage = ImageBuffer::from_fn(256, 256, |x, y| {
        //println!("generate pic... remain {}%",((x*y) as f64 / (256*256) as f64)* 100 as f64);
        let r = x as f64 / 255_f64;
        let g = y as f64 / 255_f64;
        image::Rgb([
            (r * 256_f64) as u8,
            (g * 256_f64) as u8,
            (b * 256_f64) as u8,
        ])
    });
    let v = Vec3::new(1.0,2.0,3.0);
    let x = Vec3::new(1.0,2.0,3.0);
    let s = v + x;
    println!("{:#?}",s);
    image.save("pic.png").expect("save file failed!");
}
