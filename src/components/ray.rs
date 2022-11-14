#![allow(dead_code)]

use crate::{
    types::{
        color::{Color, COLOR_BLACK, COLOR_WHITE},
        hit_record::Point3,
        vec3::Vec3,
    },
    utils::{config::Config, utilities::MAX_F64},
};

use super::traceable::{Traceable, TraceableGroup};

#[derive(Debug)]
pub struct Ray {
    d_origin: Point3,
    d_direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            d_origin: *origin,
            d_direction: *direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.d_origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.d_direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.d_origin + self.d_direction.scaled(t)
    }

    pub fn ray_color(&self, scene_objects: &TraceableGroup, scene_config: &Config) -> Color {
        self.ray_color_internal(
            scene_objects,
            scene_config,
            scene_config.rays_config().max_child_rays(),
        )
    }

    fn ray_color_internal(
        &self,
        scene_objects: &TraceableGroup,
        scene_config: &Config,
        depth: u64,
    ) -> Color {
        if depth <= 0 {
            return COLOR_BLACK;
        }

        let hit_record_option = scene_objects.intersects_ray(self, 0.00001, MAX_F64);
        if let Some(hit_record) = hit_record_option {
            if let Some((scattered_ray, attenuated_color)) =
                hit_record
                    .material()
                    .scatter(self, &hit_record, scene_config)
            {
                return scattered_ray.ray_color_internal(scene_objects, scene_config, depth - 1)
                    * attenuated_color;
            } else {
                return COLOR_BLACK;
            }
        }

        let t = (self
            .direction()
            .unit_vector()
            .expect("Weirdly, the ray is heading in the null direction")
            .y()
            + 1.0)
            * 0.5;

        let start_blend = COLOR_WHITE;
        let end_blend = Color::new(0.5, 0.7, 1.0);

        start_blend.scaled(1.0 - t) + end_blend.scaled(t)
    }
}
