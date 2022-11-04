#![allow(dead_code)]

use super::color::Color;

use super::component::Component;
use super::vec3::Vec3;

type Point3 = Vec3;

#[derive(Debug)]
pub struct Ray {
    _origin: Point3,
    _direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            _origin: *origin,
            _direction: *direction,
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
            let intersection = component.intersects_ray(self);
            if intersection.is_some() {
                let normal = (self.at(intersection.unwrap()) - *component.center())
                    .unit_vector()
                    .expect("Ray component intersection invalid");
                return Color::new(1.0 + normal.x(), 1.0 + normal.y(), 1.0 + normal.z())
                    .scaled(0.5);
            }
        }

        let t = (self
            .direction()
            .unit_vector()
            .expect("Weirdly, the ray is heading in the null direction")
            .y()
            + 1.0)
            * 0.5;

        let start_blend = Color::new(1.0, 1.0, 1.0);
        let end_blend = Color::new(0.5, 0.7, 1.0);

        start_blend.scaled(1.0 - t) + end_blend.scaled(t)
    }
}
