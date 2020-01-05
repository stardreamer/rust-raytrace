use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

use super::Scatterable;
use crate::obj::HitRecord;

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub reflection_index: f64,
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut impl Iterator<Item = f64>,
    ) -> Option<(Ray, Vec3)> {
        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(&hit.n) > 0_f64 {
            (
                -1_f64 * hit.n,
                self.reflection_index,
                self.reflection_index * ray.direction.dot(&hit.n) / ray.direction.length(),
            )
        } else {
            (
                hit.n,
                1_f64 / self.reflection_index,
                -ray.direction.dot(&hit.n) / ray.direction.length(),
            )
        };

        let (refracted, reflection_probability) =
            match super::refract(&ray.direction, &outward_normal, ni_over_nt) {
                Some(refracted) => (
                    Some(Ray::new(hit.p, refracted)),
                    super::schlick(cosine, self.reflection_index),
                ),
                None => (None, 1_f64),
            };

        if refracted.is_none() {
            return Some((
                Ray::new(hit.p, super::reflect(&ray.direction, &hit.n)),
                Vec3::new(1_f64, 1_f64, 1_f64),
            ));
        } else {
            if rng.next().unwrap() < reflection_probability {
                return Some((
                    Ray::new(hit.p, super::reflect(&ray.direction, &hit.n)),
                    Vec3::new(1_f64, 1_f64, 1_f64),
                ));
            } else {
                return Some((refracted.unwrap(), Vec3::new(1_f64, 1_f64, 1_f64)));
            }
        }
    }
}
