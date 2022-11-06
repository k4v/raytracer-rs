#![allow(dead_code)]

// Constants
pub const MAX_F: f64 = std::f64::MAX;
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
