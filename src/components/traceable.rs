#![allow(dead_code)]

use crate::types::vec3::Vec3;

use super::ray::Ray;

pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    trace: f64,
    point: Point3,
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    pub fn new(trace: f64, point: Point3, normal: Vec3, front_face: bool) -> Self {
        HitRecord {
            trace: trace,
            point: point,
            normal: normal,
            front_face: front_face,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            outward_normal.scaled(-1.0)
        };
    }

    pub fn point(&self) -> &Point3 {
        return &self.point;
    }

    pub fn normal(&self) -> &Vec3 {
        return &self.normal;
    }

    pub fn is_front_facing(&self) -> bool {
        return self.front_face;
    }
}

pub trait Traceable {
    fn intersects_ray(&self, ray: &Ray, min_trace: f64, max_trace: f64) -> Option<HitRecord>;
}

pub struct TraceableGroup {
    pub objects: Vec<Box<dyn Traceable>>,
}

impl Traceable for TraceableGroup {
    fn intersects_ray(&self, ray: &Ray, min_trace: f64, max_trace: f64) -> Option<HitRecord> {
        if self.objects.len() == 0 {
            return None;
        }

        let mut hit_record: Option<HitRecord> = None;

        let mut any_intersect = false;
        let mut closest_intersect = max_trace;

        for object in &self.objects {
            let this_hit_record = object.intersects_ray(ray, min_trace, closest_intersect);
            if this_hit_record.is_some() {
                any_intersect = true;
                closest_intersect = this_hit_record.unwrap().trace;

                hit_record = this_hit_record;
            }
        }

        return if any_intersect { hit_record } else { None };
    }
}

impl TraceableGroup {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Traceable>) {
        self.objects.push(object);
    }
}
