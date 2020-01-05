use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

use super::Scatterable;
use crate::obj::HitRecord;

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut impl Iterator<Item = f64>,
    ) -> Option<(Ray, Vec3)> {
        let reflected = super::reflect(
            &(ray.direction.unit_vector() + self.fuzz * super::random_in_unit_sphere(rng)),
            &hit.n,
        );

        Some((Ray::new(hit.p, reflected), self.albedo))
    }
}
