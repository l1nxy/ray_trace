use crate::{
    color::Color, ray::Ray, sphere::HitRecord, utils::random_in_unit_sphere, utils::*, vec3::*,
};

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
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
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = ray.dir.unit().reflect(rec.normal);
        let fuzz_ray = reflected + random_in_unit_sphere() * self.fuzz;
        *scattered = Ray::new(&rec.p, &fuzz_ray);
        *color = self.albedo;
        Vec3::dot(scattered.dir, rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(_refracted: f64) -> Self {
        Self { ir: _refracted }
    }

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let r0 = ((1.0 - ref_index) / (1.0 + ref_index)).powi(2);
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        _color: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        *_color = Color::new(1.0, 1.0, 1.0);
        let (_refaction_ratio, _out_normal) = if Vec3::dot(rec.normal, ray.dir) < 0.0 {
            (1.0 / self.ir, rec.normal)
        } else {
            (self.ir, 0.0 - rec.normal)
        };

        let _unit_dir = ray.dir.unit();
        let _cos_theta = Vec3::dot(Vec3::new(0.0, 0.0, 0.0) - ray.dir, _out_normal).min(1.0);
        let _sin_theta = (1.0 - _cos_theta.powi(2)).sqrt();

        let cannot_refract = _refaction_ratio * _sin_theta > 1.0;

        *_scattered = if cannot_refract
            || Dielectric::reflectance(_cos_theta, _refaction_ratio) > get_random_number()
        {
            Ray::new(&rec.p, &_unit_dir.reflect(_out_normal))
        } else {
            Ray::new(&rec.p, &_unit_dir.refract(_out_normal, _refaction_ratio))
        };
        true
    }
}
