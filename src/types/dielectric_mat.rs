#![allow(dead_code)]

use crate::components::ray::Ray;
use crate::utils::config::Config;
use crate::utils::utilities::{fmin, random_f64};

use super::color::{Color, COLOR_WHITE};
use super::hit_record::HitRecord;
use super::material::Material;
use super::vec3::Vec3;

pub const IOR_AIR: f64 = 1.0;
pub const IOR_ICE: f64 = 1.309;
pub const IOR_WATER: f64 = 1.325;
pub const IOR_GLASS: f64 = 1.5;
pub const IOR_DIAMOND: f64 = 2.418;

#[derive(Clone)]
pub struct DielectricMaterial {
    d_ior: f64,
}

impl DielectricMaterial {
    pub fn new(ior: f64) -> Self {
        DielectricMaterial { d_ior: ior }
    }

    fn reflectance(cos_theta: f64, relative_ior: f64) -> f64 {
        // Use Schlick's approximation for reflectance,
        // to address varying reflectivity by ray angle
        let r0 = (1.0 - relative_ior) / (1.0 + relative_ior);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * ((1.0 - cos_theta).powi(5));
    }
}

impl Material for DielectricMaterial {
    fn scatter(
        &self,
        parent_ray: &Ray,
        hit_record: &HitRecord,
        _scene_config: &Config,
    ) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit_record.is_front_facing() {
            1.0 / self.d_ior
        } else {
            self.d_ior
        };

        let cos_theta = fmin(
            hit_record
                .normal()
                .dot(&parent_ray.direction().unit_vector().unwrap())
                * -1.0,
            1.0,
        );
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let new_ray_direction: Vec3;

        if cannot_refract
            || DielectricMaterial::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            new_ray_direction = parent_ray
                .direction()
                .unit_vector()
                .unwrap()
                .reflect(hit_record.normal());
        } else {
            new_ray_direction = parent_ray
                .direction()
                .unit_vector()
                .unwrap()
                .refract(hit_record.normal(), refraction_ratio);
        }

        let scattered_ray = Ray::new(hit_record.point(), &new_ray_direction);
        Some((scattered_ray, COLOR_WHITE))
    }
}
