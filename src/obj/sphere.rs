use super::HitRecord;
use super::Hittable;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn hit_test() {
        let s = Sphere::new(Vec3::new(0_f64, 0_f64, -1_f64), 0.5);
        let r = Ray::new(Vec3::default(), s.center);
        let m_r = Ray::new(Vec3::default(), -1_f64 * s.center);

        let hit = s.hit(&r, 0_f64, std::f64::MAX);
        let miss = s.hit(&m_r, 0_f64, std::f64::MAX);

        assert_eq!(hit.is_some(), true);
        assert_eq!(miss.is_none(), true);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0_f64 {
            let t_fi = (-b - discriminant.sqrt()) / a;

            if t_fi > t_min && t_fi < t_max {
                let p_fi = ray.point_at(t_fi);
                return Some(HitRecord {
                    t: t_fi,
                    p: p_fi,
                    n: (p_fi - self.center) / self.radius,
                });
            }

            let t_si = (-b + discriminant.sqrt()) / a;

            if t_si > t_min && t_si < t_max {
                let p_si = ray.point_at(t_si);
                return Some(HitRecord {
                    t: t_si,
                    p: p_si,
                    n: (p_si - self.center) / self.radius,
                });
            }
        }
        None
    }
}
