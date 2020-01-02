use super::vec3::Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_test() {
        assert_eq!(
            Ray {
                origin: Vec3::new(0_f64, 0_f64, 0_f64),
                direction: Vec3::new(1_f64, 1_f64, 1_f64)
            }
            .point_at(4_f64),
            Vec3::new(4_f64, 4_f64, 4_f64)
        );
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}
