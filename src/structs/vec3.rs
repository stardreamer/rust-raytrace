use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_test() {
        assert_eq!(
            Vec3 {
                components: [1_f64, 1_f64, 1_f64]
            } + Vec3 {
                components: [1_f64, 2_f64, 3_f64]
            },
            Vec3 {
                components: [2_f64, 3_f64, 4_f64]
            }
        );
    }

    #[test]
    fn sub_test() {
        assert_eq!(
            Vec3 {
                components: [1_f64, 1_f64, 1_f64]
            } - Vec3 {
                components: [1_f64, 2_f64, 3_f64]
            },
            Vec3 {
                components: [0_f64, -1_f64, -2_f64]
            }
        );
    }

    #[test]
    fn mul_test() {
        assert_eq!(
            Vec3 {
                components: [1_f64, 1_f64, 1_f64]
            } * Vec3 {
                components: [1_f64, 2_f64, 3_f64]
            },
            Vec3 {
                components: [1_f64, 2_f64, 3_f64]
            }
        );
    }

    #[test]
    fn mul_scalar_test() {
        assert_eq!(
            Vec3 {
                components: [1_f64, 1_f64, 1_f64]
            } * 2_f64,
            Vec3 {
                components: [2_f64, 2_f64, 2_f64]
            }
        );
    }

    #[test]
    fn mul_left_scalar_test() {
        assert_eq!(
            4_f64
                * Vec3 {
                    components: [1_f64, 2_f64, 3_f64]
                },
            Vec3 {
                components: [4_f64, 8_f64, 12_f64]
            }
        );
    }

    #[test]
    fn div_test() {
        assert_eq!(
            Vec3 {
                components: [2_f64, 4_f64, 6_f64]
            } / Vec3 {
                components: [2_f64, 2_f64, 2_f64]
            },
            Vec3 {
                components: [1_f64, 2_f64, 3_f64]
            }
        );
    }

    #[test]
    fn div_scalar_test() {
        assert_eq!(
            Vec3 {
                components: [2_f64, 4_f64, 6_f64]
            } / 2_f64,
            Vec3 {
                components: [1_f64, 2_f64, 3_f64]
            }
        );
    }

    #[test]
    fn div_left_scalar_test() {
        assert_eq!(
            4_f64
                / Vec3 {
                    components: [1_f64, 2_f64, 4_f64]
                },
            Vec3 {
                components: [4_f64, 2_f64, 1_f64]
            }
        );
    }

    #[test]
    fn length2_test() {
        assert_eq!(
            (Vec3 {
                components: [1_f64, 1_f64, 1_f64]
            })
            .length2(),
            3_f64
        );

        assert_eq!(
            (Vec3 {
                components: [3_f64, 4_f64, 12_f64]
            })
            .length2(),
            169_f64
        );
    }

    #[test]
    fn length_test() {
        assert_eq!(
            (Vec3 {
                components: [3_f64, 4_f64, 12_f64]
            })
            .length(),
            13_f64
        );
    }

    #[test]
    fn add_assign_test() {
        let mut a = Vec3::new(1_f64, 2_f64, 3_f64);
        let b = Vec3::new(-1_f64, -2_f64, -3_f64);

        a += b;
        assert_eq!(a, Vec3::new(0_f64, 0_f64, 0_f64));
    }

    #[test]
    fn sub_assign_test() {
        let mut a = Vec3::new(1_f64, 2_f64, 3_f64);
        let b = Vec3::new(-1_f64, -2_f64, -3_f64);

        a -= b;
        assert_eq!(a, Vec3::new(2_f64, 4_f64, 6_f64));
    }

    #[test]
    fn mul_assign_test() {
        let mut a = Vec3::new(1_f64, 2_f64, 3_f64);
        let b = Vec3::new(-1_f64, -1_f64, -1_f64);

        a *= b;
        assert_eq!(a, Vec3::new(-1_f64, -2_f64, -3_f64));
    }

    #[test]
    fn mul_assign_scalar_test() {
        let mut a = Vec3::new(1_f64, 2_f64, 3_f64);

        a *= -1_f64;
        assert_eq!(a, Vec3::new(-1_f64, -2_f64, -3_f64));
    }

    #[test]
    fn div_assign_test() {
        let mut a = Vec3::new(1_f64, 2_f64, 3_f64);
        let b = Vec3::new(-1_f64, -1_f64, -1_f64);

        a /= b;
        assert_eq!(a, Vec3::new(-1_f64, -2_f64, -3_f64));
    }

    #[test]
    fn div_assign_scalar_test() {
        let mut a = Vec3::new(1_f64, 2_f64, 3_f64);

        a /= -1_f64;
        assert_eq!(a, Vec3::new(-1_f64, -2_f64, -3_f64));
    }

    #[test]
    fn unitize_test() {
        let mut a = Vec3::new(0_f64, 0_f64, 4_f64);

        a.unitize();

        assert_eq!(a, Vec3::new(0_f64, 0_f64, 1_f64));
    }

    #[test]
    fn unit_vector_test() {
        let a = Vec3::new(0_f64, 0_f64, 4_f64);

        let b = a.unit_vector();

        assert_eq!(b, Vec3::new(0_f64, 0_f64, 1_f64));
    }

    #[test]
    fn dot_test() {
        let a = Vec3::new(1_f64, 2_f64, 3_f64);
        let b = Vec3::new(4_f64, 5_f64, 6_f64);

        assert_eq!(a.dot(&b), 32_f64);
    }

    #[test]
    fn cross_test() {
        let a = Vec3::new(2_f64, 3_f64, 4_f64);
        let b = Vec3::new(5_f64, 6_f64, 7_f64);

        assert_eq!(a.cross(&b), Vec3::new(-3_f64, 6_f64, -3_f64));
    }

    #[test]
    fn default_test() {
        assert_eq!(Vec3::default(), Vec3::new(0_f64, 0_f64, 0_f64));
    }

    #[test]
    fn rgb_test() {
        let v = Vec3::new(12_f64, 13_f64, 14_f64);

        let (r, g, b) = v.irgb(1_f64);

        assert_eq!(r, 12_i64);
        assert_eq!(g, 13_i64);
        assert_eq!(b, 14_i64);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub components: [f64; 3],
}

impl Vec3 {
    pub fn new(c1: f64, c2: f64, c3: f64) -> Vec3 {
        Vec3 {
            components: [c1, c2, c3],
        }
    }

    pub fn unitize(&mut self) {
        *self /= self.length();
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn irgb(&self, scale_factor: f64) -> (i64, i64, i64) {
        (
            (self.r() * scale_factor) as i64,
            (self.g() * scale_factor) as i64,
            (self.b() * scale_factor) as i64,
        )
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            components: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    pub fn x(&self) -> f64 {
        self.components[0]
    }

    pub fn y(&self) -> f64 {
        self.components[1]
    }

    pub fn z(&self) -> f64 {
        self.components[2]
    }

    pub fn r(&self) -> f64 {
        self.components[0]
    }

    pub fn g(&self) -> f64 {
        self.components[1]
    }

    pub fn b(&self) -> f64 {
        self.components[2]
    }

    pub fn length2(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            components: [0_f64, 0_f64, 0_f64],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            components: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            components: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            components: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            components: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            components: [self * rhs.x(), self * rhs.y(), self * rhs.z()],
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            components: [self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z()],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            components: [self.x() / rhs, self.y() / rhs, self.z() / rhs],
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            components: [self / rhs.x(), self / rhs.y(), self / rhs.z()],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.components[0] += other.components[0];
        self.components[1] += other.components[1];
        self.components[2] += other.components[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.components[0] -= other.components[0];
        self.components[1] -= other.components[1];
        self.components[2] -= other.components[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.components[0] *= other.components[0];
        self.components[1] *= other.components[1];
        self.components[2] *= other.components[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.components[0] /= other.components[0];
        self.components[1] /= other.components[1];
        self.components[2] /= other.components[2];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.components[0] /= rhs;
        self.components[1] /= rhs;
        self.components[2] /= rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.components[0] *= rhs;
        self.components[1] *= rhs;
        self.components[2] *= rhs;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}
