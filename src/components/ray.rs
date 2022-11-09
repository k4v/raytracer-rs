#![allow(dead_code)]

use crate::{
    types::{color::Color, vec3::Vec3},
    utils::{
        config::{Config, DiffuseScatterMode},
        utilities::{random_point_in_hemisphere, random_point_in_unit_sphere, MAX_F64},
    },
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
            return Color::new(0.0, 0.0, 0.0);
        }

        let hit_record_option = scene_objects.intersects_ray(self, 0.00001, MAX_F64);
        if let Some(hit_record) = hit_record_option {
            let point = hit_record.point();
            let normal = hit_record.normal();

            let diffuse_scatter_mode = match scene_config.rays_config().diffuse_scatter_mode() {
                DiffuseScatterMode::ApproxLambert => random_point_in_unit_sphere(),
                DiffuseScatterMode::TrueLambert => {
                    random_point_in_unit_sphere().unit_vector().unwrap()
                }
                DiffuseScatterMode::Hemispherical => random_point_in_hemisphere(normal),
            };

            let target = *point + *normal + diffuse_scatter_mode;
            let child_ray = Ray::new(point, &(target - *point));

            return child_ray
                .ray_color_internal(scene_objects, scene_config, depth - 1)
                .scaled(0.5);
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
