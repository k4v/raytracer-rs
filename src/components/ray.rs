#![allow(dead_code)]

use crate::{
    types::{color::Color, vec3::Vec3},
    utils::utilities::MAX_F,
};

use super::traceable::{Point3, Traceable, TraceableGroup};

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

    pub fn ray_color(&self, scene_objects: &TraceableGroup) -> Color {
        let hit_record = scene_objects.intersects_ray(self, 0.0, MAX_F);
        if hit_record.is_some() {
            return (hit_record.unwrap().normal().to_owned() + Color::ones_vec()).scaled(0.5);
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
