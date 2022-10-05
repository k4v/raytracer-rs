#![allow(dead_code)]

use super::color::Color;

use super::vec3::Vec3;

type Point3 = Vec3;

pub struct Ray {
    _origin: Point3,
    _direction: Vec3
}

impl Ray {
    pub fn new() -> Self {
        Self {
            _origin: Point3::new(),
            _direction: Vec3::new()
        }
    }

    pub fn from(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            _origin: origin.clone(),
            _direction: direction.clone()
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self._origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self._direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self._origin + self._direction.scaled(t)
    }

    pub fn ray_color(&self) -> Color {
        let unit_direction = self.direction().unit_vec();
        let t = 0.5*(unit_direction.y() + 1.0);
        return Color::from(1.0, 1.0, 1.0).scaled(1.0-t) + Color::from(0.5, 0.7, 1.0).scaled(t);
    }
}
