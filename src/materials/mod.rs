use crate::obj::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub mod lambertian;
pub mod metal;

pub trait Scatterable {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut impl Iterator<Item = f64>,
    ) -> Option<(Ray, Vec3)>;
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Metal(metal::Metal),
    Lambertian(lambertian::Lambertian),
}

impl Scatterable for Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut impl Iterator<Item = f64>,
    ) -> Option<(Ray, Vec3)> {
        match *self {
            Material::Lambertian(ref mat) => mat.scatter(ray, hit, rng),
            Material::Metal(ref mat) => mat.scatter(ray, hit, rng),
        }
    }
}

fn random_in_unit_sphere(rng: &mut impl Iterator<Item = f64>) -> Vec3 {
    let mut r_vec = Vec3::new(2_f64, 2_f64, 2_f64);

    while r_vec.length2() >= 1_f64 {
        r_vec = Vec3::new(
            2_f64 * rng.next().unwrap() - 1_f64,
            2_f64 * rng.next().unwrap() - 1_f64,
            2_f64 * rng.next().unwrap() - 1_f64,
        );
    }

    r_vec
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(lambertian::Lambertian {
            albedo: Vec3::new(0.9_f64, 0.5_f64, 0.5_f64),
        })
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2_f64 * v.dot(n) * *n
}
