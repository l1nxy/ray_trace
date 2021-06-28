use super::sphere::*;
use super::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anythings = false;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter() {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anythings = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }
        hit_anythings
    }
}
