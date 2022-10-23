#![allow(dead_code)]

use super::color::Color;

use super::component::Component;
use super::vec3::Vec3;

type Point3 = Vec3;

#[derive(Debug)]
pub struct Ray {
    _origin: Point3,
    _direction: Vec3
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
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

    pub fn ray_color(&self, components: &Vec<Box<dyn Component>>) -> Color {

        for component in components {
            if component.intersects_ray(&self) {
                return Color::new(1.0, 0.0, 0.0);
            }
        }

        let unit_direction = self.direction().unit_vector();
        let t = (unit_direction.y() + 1.0)*0.5;

        let start_blend = Color::new(1.0, 1.0, 1.0);
        let end_blend = Color::new(0.5, 0.7, 1.0);

        return start_blend.scaled(1.0-t) + end_blend.scaled(t);
    }
}
