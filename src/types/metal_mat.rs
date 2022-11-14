#![allow(dead_code)]

use crate::utils::config::Config;
use crate::utils::utilities::clamp;
use crate::{components::ray::Ray, utils::utilities::random_point_in_unit_sphere};

use super::{color::Color, material::Material};

#[derive(Clone)]
pub struct MetalMaterial {
    d_albedo: Color,
    d_roughness: f64,
}

impl MetalMaterial {
    pub fn new(albedo: &Color, roughness: f64) -> Self {
        MetalMaterial {
            d_albedo: *albedo,
            d_roughness: clamp(roughness, 0.0, 1.0),
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        parent_ray: &crate::components::ray::Ray,
        hit_record: &super::hit_record::HitRecord,
        _scene_config: &Config,
    ) -> Option<(Ray, Color)> {
        let reflected_direction = parent_ray
            .direction()
            .unit_vector()
            .unwrap()
            .reflect(hit_record.normal());

        let scattered_ray = Ray::new(
            hit_record.point(),
            &(reflected_direction + random_point_in_unit_sphere().scaled(self.d_roughness)),
        );

        if reflected_direction.dot(hit_record.normal()) > 0.0 {
            Some((scattered_ray, self.d_albedo))
        } else {
            None
        }
    }
}
