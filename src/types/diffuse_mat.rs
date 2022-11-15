#[allow(dead_code)]
use crate::components::ray::Ray;
use crate::utils::config::{Config, DiffuseScatterMode};
use crate::utils::utilities::{random_point_in_hemisphere, random_point_in_unit_sphere};

use super::{color::Color, material::Material};

#[derive(Clone)]
pub struct DiffuseMaterial {
    d_albedo: Color,
}

impl DiffuseMaterial {
    pub fn new(albedo: &Color) -> Self {
        DiffuseMaterial { d_albedo: *albedo }
    }
}

impl Material for DiffuseMaterial {
    fn scatter(
        &self,
        _parent_ray: &crate::components::ray::Ray,
        hit_record: &super::hit_record::HitRecord,
        scene_config: &Config,
    ) -> Option<(Ray, Color)> {
        let diffuse_scatter_mode = match scene_config.rays_config().diffuse_scatter_mode() {
            DiffuseScatterMode::ApproxLambert => random_point_in_unit_sphere(),
            DiffuseScatterMode::TrueLambert => random_point_in_unit_sphere().unit_vector().unwrap(),
            DiffuseScatterMode::Hemispherical => random_point_in_hemisphere(hit_record.normal()),
        };

        let mut scatter_direction = *hit_record.normal() + diffuse_scatter_mode;

        if scatter_direction.is_nearly_zero() {
            scatter_direction = *hit_record.normal()
        }

        Some((Ray::new(hit_record.point(), &scatter_direction), self.d_albedo))
    }
}
