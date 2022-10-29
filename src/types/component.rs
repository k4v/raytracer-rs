#![allow(dead_code)]

use super::{ray::Ray, vec3::Vec3};

pub trait Component {
    fn center(&self) -> &Vec3;
    fn radius(&self) -> f64;
    fn intersects_ray(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    _center: Vec3,
    _radius: f64,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Self {
        Sphere {
            _center: center.clone(),
            _radius: radius,
        }
    }
}

impl Component for Sphere {
    fn center(&self) -> &Vec3 {
        &self._center
    }

    fn radius(&self) -> f64 {
        self._radius
    }

    fn intersects_ray(&self, ray: &Ray) -> bool {
        let ray_trace = *ray.origin() - self._center;
        let a = ray.direction().len_squared();
        let b = ray_trace.dot(ray.direction());
        let c = ray_trace.len_squared() - (self._radius * self._radius);

        let discriminant = (b * b) - (a * c);

        discriminant > 0.0
    }
}
