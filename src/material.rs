use crate::{color::Color, ray::Ray, sphere::HitRecord, utils::random_in_unit_sphere, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = rec.normal + random_in_unit_sphere().unit();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *scattered = Ray::new(&rec.p, &scatter_dir);
        *color = self.albedo;
        return true;
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = ray.dir.unit().reflect(rec.normal);
        *scattered = Ray::new(&rec.p, &reflected);
        *color = self.albedo;
        Vec3::dot(scattered.dir, rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}
