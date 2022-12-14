#[cfg(test)]
mod tests {
    use crate::{
        components::{ray::Ray, sphere::Sphere, traceable::Traceable},
        types::{color::Color, diffuse_mat::DiffuseMaterial, vec3::Vec3},
        utils::utilities::MAX_F64,
    };

    #[test]
    fn test_instantiation_and_getters() {
        let center = Vec3::new(0.0, 1.0, -2.0);

        let sphere =
            Sphere::new(&center, 1.0, Box::new(DiffuseMaterial::new(&Color::new(0.7, 0.3, 0.3))));
        assert!(sphere.is_ok());
        assert_eq!(sphere.as_ref().unwrap().radius(), 1.0);
        assert_eq!(sphere.as_ref().unwrap().center(), &Vec3::new(0.0, 1.0, -2.0));

        let sphere =
            Sphere::new(&center, -5.0, Box::new(DiffuseMaterial::new(&Color::new(0.7, 0.3, 0.3))));
        assert!(sphere.is_err());
    }

    #[test]
    fn test_intersects_ray() {
        let center = &Vec3::new(0.0, 1.0, -2.0);

        let sphere =
            Sphere::new(&center, 1.0, Box::new(DiffuseMaterial::new(&Color::new(0.7, 0.3, 0.3))));
        assert!(sphere.is_ok());

        // Ray inside sphere
        {
            let ray = Ray::new(&center, &Vec3::new(0.0, 0.0, -1.0));
            let intersection = sphere.as_ref().unwrap().intersects_ray(&ray, 0.0, MAX_F64);
            assert!(intersection.is_some());
        }

        // Ray tangential to sphere
        {
            let ray = Ray::new(&center, &Vec3::new(0.0, 1.0, 0.0));
            let intersection = sphere.as_ref().unwrap().intersects_ray(&ray, 0.0, MAX_F64);
            assert!(intersection.is_some());
        }

        // Ray through sphere
        {
            let ray = Ray::new(&center, &Vec3::new(1.0, 1.0, -3.0));
            assert!(sphere.unwrap().intersects_ray(&ray, 0.0, MAX_F64).is_some());
        }
    }

    #[test]
    fn test_not_intersects_ray() {
        let center = Vec3::new(0.0, 1.0, -2.0);

        let sphere =
            Sphere::new(&center, 1.0, Box::new(DiffuseMaterial::new(&Color::new(0.7, 0.3, 0.3))));
        assert!(sphere.is_ok());

        // Ray outside and away from sphere
        {
            let ray = Ray::new(&Vec3::new(0.0, 1.0, -4.0), &Vec3::new(1.0, 1.0, -2.0));
            assert!(sphere.unwrap().intersects_ray(&ray, 0.0, MAX_F64).is_none());
        }
    }
}
