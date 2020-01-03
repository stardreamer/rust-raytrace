use super::HitRecord;
use super::Hittable;
use crate::structs::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut current_hit: Option<HitRecord> = None;
        let mut best_hit_time = t_max;
        for object in &self.objects {
            let hit_candidate = object.hit(ray, t_min, best_hit_time);

            match hit_candidate {
                Some(hit) => {
                    best_hit_time = hit.t;
                    current_hit = Some(hit);
                }
                None => (),
            }
        }

        current_hit
    }
}
