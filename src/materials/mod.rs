use crate::obj::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub mod dielectric;
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

#[derive(Debug, Clone)]
pub enum Material {
    Metal(metal::Metal),
    Lambertian(lambertian::Lambertian),
    Dielectric(dielectric::Dielectric),
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
            Material::Dielectric(ref mat) => mat.scatter(ray, hit, rng),
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

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1_f64 - ni_over_nt * ni_over_nt * (1_f64 - dt * dt);

    if discriminant > 0_f64 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, reflection_index: f64) -> f64 {
    let mut r0 = (1_f64 - reflection_index) / (1_f64 + reflection_index);

    r0 = r0 * r0;

    r0 + (1_f64 - r0) * (1_f64 - cosine).powi(5)
}
