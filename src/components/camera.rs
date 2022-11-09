#![allow(dead_code)]

use crate::{types::vec3::Vec3, utils::config::Config};

use super::{ray::Ray, traceable::Point3};

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
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
            origin: *origin,
            lower_left_corner: *origin
                - Vec3::new(_horizontal.x() / 2.0, _vertical.y() / 2.0, focal_length),
            horizontal: _horizontal,
            vertical: _vertical,
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
            self.lower_left_corner + self.horizontal.scaled(u) + self.vertical.scaled(v)
                - self.origin;
        Ray::new(&self.origin, &direction)
    }
}
