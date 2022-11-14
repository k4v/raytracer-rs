#![allow(dead_code)]

use crate::components::ray::Ray;

use super::{material::Material, vec3::Vec3};

pub type Point3 = Vec3;

#[derive(Clone)]
pub struct HitRecord {
    d_trace: f64,
    d_point: Point3,
    d_normal: Vec3,
    d_front_face: bool,
    d_material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(
        trace: f64,
        point: Point3,
        normal: Vec3,
        front_face: bool,
        material: &Box<dyn Material>,
    ) -> Self {
        HitRecord {
            d_trace: trace,
            d_point: point,
            d_normal: normal,
            d_front_face: front_face,
            d_material: material.clone(),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.d_front_face = ray.direction().dot(outward_normal) < 0.0;
        self.d_normal = if self.d_front_face {
            *outward_normal
        } else {
            outward_normal.scaled(-1.0)
        };
    }

    pub fn trace(&self) -> f64 {
        self.d_trace
    }

    pub fn point(&self) -> &Point3 {
        &self.d_point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.d_normal
    }

    pub fn is_front_facing(&self) -> bool {
        self.d_front_face
    }

    pub fn material(&self) -> &Box<dyn Material> {
        &self.d_material
    }
}
