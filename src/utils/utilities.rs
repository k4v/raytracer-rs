#![allow(dead_code)]

use crate::types::vec3::Vec3;

// Constants
pub const MAX_F64: f64 = std::f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

pub fn random_f64_between(min_inclusive: f64, max_exclusive: f64) -> f64 {
    min_inclusive + (max_exclusive - min_inclusive) * random_f64()
}

pub fn random_point_in_unit_sphere() -> Vec3 {
    loop {
        let candidate = Vec3::random_between(-1.0, 1.0);
        if candidate.len_squared() < 1.0 {
            return candidate;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_point_in_unit_sphere().unit_vector().unwrap()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
