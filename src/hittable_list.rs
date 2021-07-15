use super::sphere::*;
use super::*;

#[derive(Clone, Debug)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec: Option<HitRecord> = None;
        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_rec = Some(rec);
            }
        }
        hit_rec
    }
}
