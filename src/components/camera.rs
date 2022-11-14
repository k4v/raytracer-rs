#![allow(dead_code)]

use crate::types::{hit_record::Point3, vec3::Vec3};
use crate::utils::config::Config;

use super::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    d_origin: Point3,
    d_lower_left_corner: Point3,
    d_horizontal: Vec3,
    d_vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: &Point3,
        viewport_width: u64,
        viewport_height: u64,
        focal_length: f64,
    ) -> Self {
        let _horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
        let _vertical = Vec3::new(0.0, viewport_height as f64, 0.0);

        Camera {
            d_origin: *origin,
            d_lower_left_corner: *origin
                - Vec3::new(_horizontal.x() / 2.0, _vertical.y() / 2.0, focal_length),
            d_horizontal: _horizontal,
            d_vertical: _vertical,
        }
    }

    pub fn configure(configuration: &Config) -> Self {
        Camera::new(
            configuration.camera_config().origin(),
            configuration.camera_config().viewport_width(),
            configuration.camera_config().viewport_height(),
            configuration.camera_config().focal_length(),
        )
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.d_lower_left_corner + self.d_horizontal.scaled(u) + self.d_vertical.scaled(v)
                - self.d_origin;
        Ray::new(&self.d_origin, &direction)
    }
}
