use crate::types::{hit_record::HitRecord, material::Material, vec3::Vec3};

use super::{ray::Ray, traceable::Traceable};

pub struct Sphere {
    d_center: Vec3,
    d_radius: f64,
    d_material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64, material: Box<dyn Material>) -> Result<Self, &str> {
        if radius <= 0.0 {
            return Err("Sphere radius must be greater than 0");
        }

        Ok(Sphere {
            d_center: *center,
            d_radius: radius,
            d_material: material,
        })
    }

    pub fn center(&self) -> &Vec3 {
        &self.d_center
    }

    pub fn radius(&self) -> f64 {
        self.d_radius
    }
}

impl Traceable for Sphere {
    fn intersects_ray(&self, ray: &Ray, min_trace: f64, max_trace: f64) -> Option<HitRecord> {
        let ray_trace = *ray.origin() - *self.center();
        let a = ray.direction().len_squared();
        let b = ray_trace.dot(ray.direction());
        let c = ray_trace.len_squared() - (self.radius() * self.radius());

        let discriminant = (b * b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }

        let discriminant = discriminant.sqrt();
        let mut root = (-b - discriminant) / a;
        if root < min_trace || max_trace < root {
            root = (-b + discriminant) / a;
            if root < min_trace || max_trace < root {
                return None;
            }
        }

        let _point = ray.at(root);
        let _normal = (_point - *self.center()).scaled(1.0 / self.radius());

        let mut hit_record = HitRecord::new(root, _point, _normal, false, &self.d_material);
        hit_record.set_face_normal(ray, &_normal);
        Some(hit_record)
    }
}
