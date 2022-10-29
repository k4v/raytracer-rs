#[cfg(test)]
mod tests {
    use crate::types::vec3::Vec3;

    #[test]
    fn test_instantiation() {
        // Instantiation using Vec3::new()
        let new_v: Vec3 = Vec3::new(0.0, 1.0, -2.0);
        assert_eq!(new_v.x(), 0.0);
        assert_eq!(new_v.y(), 1.0);
        assert_eq!(new_v.z(), -2.0);

        // Instantiation using Vec3::copy()
        let copy_v: Vec3 = Vec3::copy(&new_v);
        assert_eq!(new_v, copy_v);

        let zero_v: Vec3 = Vec3::zero_vec();
        assert_eq!(zero_v, Vec3::new(0.0, 0.0, 0.0));

        let unit_v: Vec3 = Vec3::ones_vec();
        assert_eq!(unit_v, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_getters() {
        let new_v: Vec3 = Vec3::new(0.0, 1.0, -2.0);
        assert_eq!(new_v.x(), 0.0);
        assert_eq!(new_v.y(), 1.0);
        assert_eq!(new_v.z(), -2.0);
    }

    #[test]
    fn test_length() {
        let new_v: Vec3 = Vec3::new(0.0, 3.0, -4.0);
        assert_eq!(new_v.len_squared(), 25.0);
        assert_eq!(new_v.len(), 5.0);
    }

    #[test]
    fn test_scaling() {
        let new_v: Vec3 = Vec3::new(0.0, 1.0, -2.0);
        assert_eq!(new_v.scaled(1.0), new_v);
        assert_eq!(new_v.scaled(0.0), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(new_v.scaled(2.0), Vec3::new(0.0, 2.0, -4.0));
        assert_eq!(new_v.scaled(-4.0), Vec3::new(0.0, -4.0, 8.0));
        assert_eq!(new_v.scaled(0.5), Vec3::new(0.0, 0.5, -1.0));

        let mut mut_v: Vec3 = Vec3::new(0.0, 1.0, -2.0);
        mut_v.scale(1.0);
        assert_eq!(mut_v, Vec3::new(0.0, 1.0, -2.0));

        mut_v.scale(2.0);
        assert_eq!(mut_v, Vec3::new(0.0, 2.0, -4.0));

        mut_v.scale(0.5);
        assert_eq!(mut_v, Vec3::new(0.0, 1.0, -2.0));

        mut_v.scale(-2.0);
        assert_eq!(mut_v, Vec3::new(0.0, -2.0, 4.0));

        mut_v.scale(0.0);
        assert_eq!(mut_v, Vec3::zero_vec());
    }

    #[test]
    fn test_dot() {
        let v1: Vec3 = Vec3::new(0.0, 1.0, -2.0);
        assert_eq!(v1.dot(&v1), v1.len_squared());

        {
            let v2: Vec3 = Vec3::new(1.0, -5.0, 2.0);
            assert_eq!(v1.dot(&v2), -9.0);
        }

        {
            let v2: Vec3 = Vec3::new(1.0, 1.0, 1.0);
            assert_eq!(v1.dot(&v2), -1.0);
        }

        {
            let v2: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            assert_eq!(v1.dot(&v2), 0.0);
        }
    }

    #[test]
    fn test_cross() {
        let v1: Vec3 = Vec3::new(0.0, 1.0, -2.0);
        assert_eq!(v1.cross(&v1), Vec3::new(0.0, 0.0, 0.0));

        {
            let v2: Vec3 = Vec3::new(1.0, -5.0, 2.0);
            assert_eq!(v1.cross(&v2), Vec3::new(-8.0, -2.0, -1.0));
        }

        {
            let v2: Vec3 = Vec3::new(1.0, 1.0, 1.0);
            assert_eq!(v1.cross(&v2), Vec3::new(3.0, -2.0, -1.0));
        }

        {
            let v2: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            assert_eq!(v1.cross(&v2), Vec3::zero_vec());
        }
    }
}
