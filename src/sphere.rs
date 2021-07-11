use super::ray::*;
use super::vec3::Vec3;
use crate::material::*;
use std::borrow::Borrow;
use std::rc::Rc;
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
//#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(oc, ray.dir);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        let root1 = (-half_b - discriminant.sqrt()) / a;
        let root2 = (-half_b + discriminant.sqrt()) / a;

        let ret1 = root1 > t_min && root1 < t_max;
        let ret2 = root2 > t_min && root2 < t_max;

        if discriminant > 0.0 && (ret1 || ret2) {
            let t = if ret1 { root1 } else { root2 };
            let p = ray.at(t);
            let normal = (p - self.center) / self.radius;
            Some(HitRecord {
                t,
                p,
                normal,
                mat: self.mat.borrow(),
            })
        } else {
            None
        }
    }
}
