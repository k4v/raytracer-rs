#![allow(dead_code)]

use std::{ops::{Neg, AddAssign, MulAssign, DivAssign, Add, Sub, Mul}};
use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3 {
    _x: f64,
    _y: f64,
    _z: f64
}

impl Vec3 {

    /// Create a new empty Vec3
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            _x: x,
            _y: y,
            _z: z
        }
    }

    /// Create a new Vec3 copying fields from given Vec3
    pub const fn copy(v: &Vec3) -> Self {
        Vec3 {
            _x: v._x,
            _y: v._y,
            _z: v._z
        }
    }

    pub fn x(&self) -> f64 {
        self._x
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self._x
    }
    
    pub fn y(&self) -> f64 {
        self._y
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self._y
    }
    
    pub fn z(&self) -> f64 {
        self._z
    }
    
    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self._z
    }

    pub fn len_squared(&self) -> f64 {
        (self._x * self._x) + (self._y + self._y) + (self._z * self._z)
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
            _z: self._z * factor
        }
    }

    pub fn unit_vec(&self) -> Vec3 {
        let len = self.len();
        Vec3 {
            _x: self._x/len,
            _y: self._y/len,
            _z: self._z/len
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self._x * rhs._x) + (self._y * rhs._y) + (self._z * rhs._z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            _x: self._y * rhs._z - self._z * rhs._y,
            _y: self._z * rhs._x - self._x * rhs._z,
            _z: self._x * rhs._y - self._y * rhs._x
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
            _z: -self._z
        }
    }
}

/// Implement add and assign (v+=) operation for Vec3
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

/// Implement multiply and assign (v*=) operation for Vec3
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self._x *= rhs._x;
        self._y *= rhs._y;
        self._z *= rhs._z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            _x: self._x * rhs._x,
            _y: self._y * rhs._y,
            _z: self._z * rhs._z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self._x /= rhs._x;
        self._y /= rhs._y;
        self._z /= rhs._z;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self._x, self._y, self._z)
    }
}
