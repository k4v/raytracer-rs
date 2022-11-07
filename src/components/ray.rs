#![allow(dead_code)]

use crate::{
    types::{color::Color, vec3::Vec3},
    utils::utilities::{MAX_F64, random_unit_vector},
};

use super::traceable::{Point3, Traceable, TraceableGroup};

#[derive(Debug)]
pub struct Ray {
    _origin: Point3,
    _direction: Vec3,
}

impl Ray {
    const MAX_CHILD_RAYS: usize = 50;

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
        self.ray_color_internal(scene_objects, Ray::MAX_CHILD_RAYS)
    }

    fn ray_color_internal(&self, scene_objects: &TraceableGroup, depth: usize) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let hit_record_option = scene_objects.intersects_ray(self, 0.00001, MAX_F64);
        if let Some(hit_record) = hit_record_option {
            let point = hit_record.point();
            let normal = hit_record.normal();

            let target = *point + *normal + random_unit_vector();
            let child_ray = Ray::new(point, &(target - *point));

            return child_ray.ray_color_internal(scene_objects, depth-1).scaled(0.5);
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
