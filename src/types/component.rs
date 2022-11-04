#![allow(dead_code)]

use super::{ray::Ray, vec3::Vec3};

pub trait Component {
    fn center(&self) -> &Vec3;
    fn radius(&self) -> f64;
    fn intersects_ray(&self, ray: &Ray) -> bool;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    _center: Vec3,
    _radius: f64,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Result<Self, &str> {
        if radius <= 0.0 {
            return Err("Sphere radius must be greater than 0");
        }

        Ok(Sphere {
            _center: *center,
            _radius: radius,
        })
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
        let ray_trace = *ray.origin() - *self.center();
        let a = ray.direction().len_squared();
        let b = ray_trace.dot(ray.direction());
        let c = ray_trace.len_squared() - (self._radius * self._radius);

        let discriminant = (b * b) - (a * c);
        discriminant >= 0.0
    }
}
