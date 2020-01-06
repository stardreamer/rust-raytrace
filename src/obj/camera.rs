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
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect: f64) -> Self {
        let theta = v_fov * std::f64::consts::PI / 180_f64;
        let half_height = (theta / 2_f64).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Self {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2_f64 * half_width * u,
            vertical: 2_f64 * half_height * v,
            origin: look_from,
        }
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
