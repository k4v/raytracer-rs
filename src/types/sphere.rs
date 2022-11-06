use super::{
    ray::Ray,
    traceable::{HitRecord, Traceable},
    vec3::Vec3,
};

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

    pub fn center(&self) -> &Vec3 {
        &self._center
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

        let mut hit_record = HitRecord::new(root, _point, _normal, false);
        hit_record.set_face_normal(ray, &_normal);
        Some(hit_record)
    }
}

impl Sphere {
    pub fn radius(&self) -> f64 {
        self._radius
    }
}
