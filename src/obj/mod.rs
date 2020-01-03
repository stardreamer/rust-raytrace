use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub mod hittablelist;
pub mod sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Record of hit event between hittable object and a ray
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    /// Time when the hit had happened
    pub t: f64,
    /// Point where the ray had hit the object
    pub p: Vec3,
    /// Normal to the object hit with ray in the point of the hit
    pub n: Vec3,
}
