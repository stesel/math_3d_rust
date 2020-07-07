#[macro_use]
extern crate math_3d_rust;
use math_3d_rust::utils::vector_4::Vector4;

#[cfg(test)]
mod vector_4 {
    use super::{Vector4};

    #[test]
    fn should_create_vector() {
        let v = vector!(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn should_negate_vector() {
        let v = vector!(1.0, -2.0, 3.0);
        assert_eq!(-v, vector!(-1.0, 2.0, -3.0));
    }
}
