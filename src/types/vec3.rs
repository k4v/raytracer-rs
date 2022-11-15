#![allow(dead_code)]

use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::utils::utilities::{fmin, random_f64, random_f64_between};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3 {
    d_x: f64,
    d_y: f64,
    d_z: f64,
}

impl Vec3 {
    /// Create a new Vec3 with the given coordinates
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            d_x: x,
            d_y: y,
            d_z: z,
        }
    }

    /// Create a new Vec3 copying fields from given Vec3
    pub const fn copy(v: &Vec3) -> Self {
        Vec3 {
            d_x: v.d_x,
            d_y: v.d_y,
            d_z: v.d_z,
        }
    }

    pub fn random() -> Self {
        Vec3 {
            d_x: random_f64(),
            d_y: random_f64(),
            d_z: random_f64(),
        }
    }

    pub fn random_between(min_inclusive: f64, max_exclusive: f64) -> Self {
        Vec3 {
            d_x: random_f64_between(min_inclusive, max_exclusive),
            d_y: random_f64_between(min_inclusive, max_exclusive),
            d_z: random_f64_between(min_inclusive, max_exclusive),
        }
    }

    pub const fn x(&self) -> f64 {
        self.d_x
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.d_x
    }

    pub const fn y(&self) -> f64 {
        self.d_y
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.d_y
    }

    pub const fn z(&self) -> f64 {
        self.d_z
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.d_z
    }

    pub fn len_squared(&self) -> f64 {
        (self.d_x * self.d_x) + (self.d_y * self.d_y) + (self.d_z * self.d_z)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn scale(&mut self, factor: f64) -> &mut Self {
        self.d_x *= factor;
        self.d_y *= factor;
        self.d_z *= factor;

        self
    }

    pub fn unit_vector(&self) -> Result<Vec3, &str> {
        let length = self.len();

        if length == 0.0 {
            return Err("Null vector does not have a unit vector");
        }
        Ok(Vec3 {
            d_x: self.d_x / length,
            d_y: self.d_y / length,
            d_z: self.d_z / length,
        })
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.d_x * rhs.d_x) + (self.d_y * rhs.d_y) + (self.d_z * rhs.d_z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            d_x: self.d_y * rhs.d_z - self.d_z * rhs.d_y,
            d_y: self.d_z * rhs.d_x - self.d_x * rhs.d_z,
            d_z: self.d_x * rhs.d_y - self.d_y * rhs.d_x,
        }
    }
}

/// Implementation of utility functions
impl Vec3 {
    pub fn scaled(&self, factor: f64) -> Self {
        Self {
            d_x: self.d_x * factor,
            d_y: self.d_y * factor,
            d_z: self.d_z * factor,
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - normal.scaled(2.0 * self.dot(normal))
    }

    pub fn refract(&self, normal: &Vec3, relative_ior: f64) -> Self {
        let cos_theta = fmin(self.dot(normal) * -1.0, 1.0);
        let ray_out_orth = (*self + normal.scaled(cos_theta)).scaled(relative_ior);
        let ray_out_prll = normal.scaled(-(1.0 - ray_out_orth.len_squared()).abs().sqrt());
        return ray_out_orth + ray_out_prll;
    }

    pub fn is_nearly_zero(&self) -> bool {
        self.is_nearly(0.0)
    }

    pub fn is_nearly(&self, target: f64) -> bool {
        const E: f64 = 1e-8;
        (self.d_x - target).abs() < E
            && (self.d_y - target).abs() < E
            && (self.d_z - target).abs() < E
    }

    pub const fn zero_vec() -> Self {
        Vec3 {
            d_x: 0.0,
            d_y: 0.0,
            d_z: 0.0,
        }
    }

    pub const fn ones_vec() -> Self {
        Vec3 {
            d_x: 1.0,
            d_y: 1.0,
            d_z: 1.0,
        }
    }
}

/// Implement unary minus (-v) operation for Vec3
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            d_x: -self.d_x,
            d_y: -self.d_y,
            d_z: -self.d_z,
        }
    }
}

/// Implement add and assign (v+, v+=) operation for Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.d_x += rhs.d_x;
        self.d_y += rhs.d_y;
        self.d_z += rhs.d_z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            d_x: self.d_x + rhs.d_x,
            d_y: self.d_y + rhs.d_y,
            d_z: self.d_z + rhs.d_z,
        }
    }
}

/// Implement sub and assign (v-, v-=) operation for Vec3
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.d_x -= rhs.d_x;
        self.d_y -= rhs.d_y;
        self.d_z -= rhs.d_z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            d_x: self.d_x - rhs.d_x,
            d_y: self.d_y - rhs.d_y,
            d_z: self.d_z - rhs.d_z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            d_x: self.d_x * rhs.d_x,
            d_y: self.d_y * rhs.d_y,
            d_z: self.d_z * rhs.d_z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.d_x, self.d_y, self.d_z)
    }
}
