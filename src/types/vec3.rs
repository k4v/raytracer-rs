#![allow(dead_code)]

use std::fmt::Display;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::utils::utilities::{random_f64, random_f64_between};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3 {
    _x: f64,
    _y: f64,
    _z: f64,
}

impl Vec3 {
    /// Create a new Vec3 with the given coordinates
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            _x: x,
            _y: y,
            _z: z,
        }
    }

    /// Create a new Vec3 copying fields from given Vec3
    pub const fn copy(v: &Vec3) -> Self {
        Vec3 {
            _x: v._x,
            _y: v._y,
            _z: v._z,
        }
    }

    pub fn random() -> Self {
        Vec3 {
            _x: random_f64(),
            _y: random_f64(),
            _z: random_f64()
        }
    }

    pub fn random_between(min_inclusive: f64, max_exclusive: f64) -> Self {
        Vec3 {
            _x: random_f64_between(min_inclusive, max_exclusive),
            _y: random_f64_between(min_inclusive, max_exclusive),
            _z: random_f64_between(min_inclusive, max_exclusive)
        }
    }

    pub const fn x(&self) -> f64 {
        self._x
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self._x
    }

    pub const fn y(&self) -> f64 {
        self._y
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self._y
    }

    pub const fn z(&self) -> f64 {
        self._z
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self._z
    }

    pub fn len_squared(&self) -> f64 {
        (self._x * self._x) + (self._y * self._y) + (self._z * self._z)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn scale(&mut self, factor: f64) -> &mut Self {
        self._x *= factor;
        self._y *= factor;
        self._z *= factor;

        self
    }

    pub fn scaled(&self, factor: f64) -> Self {
        Self {
            _x: self._x * factor,
            _y: self._y * factor,
            _z: self._z * factor,
        }
    }

    pub fn unit_vector(&self) -> Result<Vec3, &str> {
        let length = self.len();

        if length == 0.0 {
            return Err("Null vector does not have a unit vector");
        }
        Ok(Vec3 {
            _x: self._x / length,
            _y: self._y / length,
            _z: self._z / length,
        })
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self._x * rhs._x) + (self._y * rhs._y) + (self._z * rhs._z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            _x: self._y * rhs._z - self._z * rhs._y,
            _y: self._z * rhs._x - self._x * rhs._z,
            _z: self._x * rhs._y - self._y * rhs._x,
        }
    }

    pub const fn zero_vec() -> Self {
        Vec3 {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
        }
    }

    pub const fn ones_vec() -> Self {
        Vec3 {
            _x: 1.0,
            _y: 1.0,
            _z: 1.0,
        }
    }
}

/// Implement unary minus (-v) operation for Vec3
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            _x: -self._x,
            _y: -self._y,
            _z: -self._z,
        }
    }
}

/// Implement add and assign (v+, v+=) operation for Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self._x += rhs._x;
        self._y += rhs._y;
        self._z += rhs._z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            _x: self._x + rhs._x,
            _y: self._y + rhs._y,
            _z: self._z + rhs._z,
        }
    }
}

/// Implement sub and assign (v-, v-=) operation for Vec3
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self._x -= rhs._x;
        self._y -= rhs._y;
        self._z -= rhs._z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            _x: self._x - rhs._x,
            _y: self._y - rhs._y,
            _z: self._z - rhs._z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self._x, self._y, self._z)
    }
}
