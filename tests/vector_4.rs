extern crate math_3d_rust;
use math_3d_rust::utils::vector_4::Vector4;

#[cfg(test)]
mod vector_4 {
    use super::{Vector4};

    #[test]
    fn should_create_vector() {
        let vector_4: Vector4 = Vector4::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!(vector_4.x, 1.0);
        assert_eq!(vector_4.y, 2.0);
        assert_eq!(vector_4.z, 3.0);
        assert_eq!(vector_4.w, 0.0);
    }
}
