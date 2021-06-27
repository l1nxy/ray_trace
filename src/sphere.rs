use super::ray::*;
use vec3::Vec3;
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
}

trait Hittable {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(oc, ray.dir);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }

            hit_record.t = root;
            hit_record.p = ray.at(root);
            hit_record.normal = (hit_record.p - self.center) / self.radius;
        }
        todo!()
    }
}
