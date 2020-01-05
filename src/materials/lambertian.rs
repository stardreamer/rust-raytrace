use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

use super::Scatterable;
use crate::obj::HitRecord;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &HitRecord,
        rng: &mut impl Iterator<Item = f64>,
    ) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.n + super::random_in_unit_sphere(rng);
        Some((Ray::new(hit.p, target - hit.p), self.albedo))
    }
}
