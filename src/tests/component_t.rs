#[cfg(test)]
mod tests {
    use crate::types::{
        component::{Component, Sphere},
        ray::Ray,
        vec3::Vec3,
    };

    #[test]
    fn test_instantiation_and_getters() {
        let sphere = Sphere::new(&Vec3::new(0.0, 1.0, -2.0), 1.0);
        assert!(sphere.is_some());
        assert_eq!(sphere.as_ref().unwrap().radius(), 1.0);
        assert_eq!(sphere.as_ref().unwrap().center(), &Vec3::new(0.0, 1.0, -2.0));

        let sphere = Sphere::new(&Vec3::new(0.0, 1.0, -2.0), -5.0);
        assert!(sphere.is_none());
    }

    #[test]
    fn test_intersects_ray() {
        let sphere = Sphere::new(&Vec3::new(0.0, 1.0, -2.0), 1.0);
        assert!(sphere.is_some());

        {
            let ray = Ray::new(&Vec3::new(0.0, 1.0, -3.0), &Vec3::new(0.0, 0.0, -1.0));
            assert!(sphere.as_ref().unwrap().intersects_ray(&ray));
        }

        {
            let ray = Ray::new(&Vec3::new(0.0, 1.0, -4.0), &Vec3::new(1.0, 1.0, -2.0));
            assert!(sphere.as_ref().unwrap().intersects_ray(&ray) == false);
        }
    }
}
