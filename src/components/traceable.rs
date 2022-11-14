#![allow(dead_code)]

use crate::types::hit_record::HitRecord;

use super::ray::Ray;

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
                closest_intersect = this_hit_record.as_ref().unwrap().trace();

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
