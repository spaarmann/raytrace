use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, c: f64) -> Self {
        Self(self.0 + c, self.1 + c, self.2 + c)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, c: f64) -> Self {
        Self(self.0 - c, self.1 - c, self.2 - c)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, c: f64) -> Self {
        Self(self.0 * c, self.1 * c, self.2 * c)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3(self * vec.0, self * vec.1, self * vec.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, vec: Self) -> Self {
        Vec3(self.0 * vec.0, self.1 * vec.1, self.2 * vec.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, c: f64) -> Self {
        Self(self.0 / c, self.1 / c, self.2 / c)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, c: f64) {
        *self = Self(self.0 + c, self.1 + c, self.2 + c)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, c: f64) {
        *self = Self(self.0 - c, self.1 - c, self.2 - c)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, c: f64) {
        *self = Self(self.0 * c, self.1 * c, self.2 * c)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, c: f64) {
        *self = Self(self.0 / c, self.1 / c, self.2 / c)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Vec3 {
    pub const ZERO: Self = Vec3(0.0, 0.0, 0.0);
    pub const ONE: Self = Vec3(1.0, 1.0, 1.0);

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3(
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
            );
            if v.mag_squared() <= 1.0 {
                break v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3(
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
                0.0,
            );
            if v.mag_squared() <= 1.0 {
                break v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen::<f64>() * std::f64::consts::PI * 2.0; // TODO: TAU when stable
        let z = rng.gen::<f64>() * 2.0 - 1.0;
        let r = (1.0 - z * z).sqrt();
        Vec3(r * a.cos(), r * a.sin(), z)
    }

    pub fn mag(self) -> f64 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn normalized(self) -> Self {
        self / self.mag()
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(normal);
        let r_out_perpendicular = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perpendicular.mag_squared()).abs().sqrt() * normal;
        r_out_perpendicular + r_out_parallel
    }
}

pub struct Vec3Iterator<'a> {
    v: &'a Vec3,
    index: u8,
}

impl<'a> Iterator for Vec3Iterator<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.v.0,
            1 => self.v.1,
            2 => self.v.2,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<'a> IntoIterator for &'a Vec3 {
    type Item = f64;
    type IntoIter = Vec3Iterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Vec3Iterator { v: self, index: 0 }
    }
}

pub struct Vec3IntoIterator {
    v: Vec3,
    index: u8,
}

impl Iterator for Vec3IntoIterator {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.v.0,
            1 => self.v.1,
            2 => self.v.2,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl IntoIterator for Vec3 {
    type Item = f64;
    type IntoIter = Vec3IntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        Vec3IntoIterator { v: self, index: 0 }
    }
}
