use crate::{components::ray::Ray, utils::config::Config};

use super::{color::Color, hit_record::HitRecord};

pub trait Material: CloneableMaterial {
    fn scatter(
        &self,
        parent_ray: &Ray,
        hit_record: &HitRecord,
        scene_config: &Config,
    ) -> Option<(Ray, Color)>;
}

pub trait CloneableMaterial {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> CloneableMaterial for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
