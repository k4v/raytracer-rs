#![allow(dead_code)]

// Constants
pub const MAX_F: f64 = std::f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
