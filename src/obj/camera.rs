use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            lower_left_corner: Vec3::new(-2_f64, -1_f64, -1_f64),
            horizontal: Vec3::new(4_f64, 0_f64, 0_f64),
            vertical: Vec3::new(0_f64, 2_f64, 0_f64),
            origin: Vec3::default(),
        }
    }
}
