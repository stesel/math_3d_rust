#[macro_use]
extern crate math_3d_rust;
use math_3d_rust::utils::vector::{Vector4};

#[macro_use]
extern crate more_asserts;

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

    #[test]
    fn should_add_two_vectors() {
        let a = vector!(-1.0, 2.0, -3.0);
        let b = vector!(5.0, -6.0, 1.0);
        assert_eq!(a + b, vector!(4.0, -4.0, -2.0));
    }

    #[test]
    fn should_subtract_two_vectors() {
        let a = vector!(-1.0, 2.0, -3.0);
        let b = vector!(5.0, -6.0, 1.0);
        assert_eq!(a - b, vector!(-6.0, 8.0, -4.0));
    }

    #[test]
    fn should_multiply_vector_by_scalar() {
        let v = vector!(1.0, -2.0, 3.0);
        assert_eq!(v * 0.5, vector!(0.5, -1.0, 1.5));
    }

    #[test]
    fn should_multiply_vector_by_vector() {
        let a = vector!(-1.0, 0.0, 2.0);
        let b = vector!(2.0, 0.0, 4.0);
        assert_eq!(a * b, vector!(-2.0, 0.0, 8.0));
    }

    #[test]
    fn should_divide_vector_by_scalar() {
        let v = vector!(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, vector!(0.5, -1.0, 1.5));
    }

    #[test]
    fn should_calculate_vector_magnitude_1() {
        let v = vector!(1.0, 0.0, 0.0);
        assert_eq!(Vector4::magnitude(v), 1.0);
    }

    #[test]
    fn should_calculate_vector_magnitude_2() {
        let v = vector!(0.0, 1.0, 0.0);
        assert_eq!(Vector4::magnitude(v), 1.0);
    }

    #[test]
    fn should_calculate_vector_magnitude_3() {
        let v = vector!(0.0, 0.0, 1.0);
        assert_eq!(Vector4::magnitude(v), 1.0);
    }

    #[test]
    fn should_calculate_vector_magnitude_4() {
        let v = vector!(1.0, 5.0, 2.0);
        assert_eq!(Vector4::magnitude(v), 30.0_f32.sqrt());
    }

    #[test]
    fn should_calculate_vector_magnitude_5() {
        let v = vector!(-1.0, -4.0, -8.0);
        assert_eq!(Vector4::magnitude(v), 9.0);
    }

    #[test]
    fn should_normalize_vector() {
        let v = vector!(-1.0, -4.0, -8.0);
        assert_eq!(
            Vector4::round_to(Vector4::normalize(v), 3),
            Vector4::round_to(vector!(-0.1111, -0.4444, -0.8888), 3)
        );
    }

    #[test]
    fn should_magnitude_of_normalized_vector_1() {
        let v = vector!(-1.0, -4.0, -8.0);
        assert_eq!(Vector4::magnitude(Vector4::normalize(v)), 1.0);
    }

    #[test]
    fn should_have_positive_dot_value() {
        let a = vector!(1.0, 2.0, 3.0);
        let b = vector!(2.0, 4.0, 6.0);
        assert_gt!(Vector4::dot(a, b), 0.0);
    }

    #[test]
    fn should_have_negative_dot_value() {
        let a = vector!(1.0, 2.0, 3.0);
        let b = vector!(1.0, 2.0, -3.0);
        assert_lt!(Vector4::dot(a, b), 0.0);
    }

    #[test]
    fn should_have_0_dot_value() {
        let a = vector!(1.0, 0.0, 0.0);
        let b = vector!(0.0, 1.0, 0.0);
        assert_eq!(Vector4::dot(a, b), 0.0);
    }

    #[test]
    fn should_calculate_cross_product_1() {
        let a = vector!(1.0, 4.0, 8.0);
        let b = vector!(8.0, 4.0, 1.0);
        assert_eq!(Vector4::cross(a, b), vector!(-28.0, 63.0, -28.0));
    }

    #[test]
    fn should_calculate_cross_product_2() {
        let a = vector!(8.0, 4.0, 1.0);
        let b = vector!(1.0, 4.0, 8.0);
        assert_eq!(Vector4::cross(a, b), vector!(28.0, -63.0, 28.0));
    }
}
