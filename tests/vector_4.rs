#[macro_use]
extern crate math_3d_rust;
use math_3d_rust::utils::vector_4::{Vector4, normalize, magnitude, round_to};

#[cfg(test)]
mod vector_4 {
    use super::{Vector4, normalize, magnitude, round_to};

    #[test]
    fn should_create_vector() {
        let v = vector4!(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn should_negate_vector() {
        let v = vector4!(1.0, -2.0, 3.0);
        assert_eq!(-v, vector4!(-1.0, 2.0, -3.0));
    }

    #[test]
    fn should_add_two_vectors() {
        let a = vector4!(-1.0, 2.0, -3.0);
        let b = vector4!(5.0, -6.0, 1.0);
        assert_eq!(a + b, vector4!(4.0, -4.0, -2.0));
    }

    #[test]
    fn should_subtract_two_vectors() {
        let a = vector4!(-1.0, 2.0, -3.0);
        let b = vector4!(5.0, -6.0, 1.0);
        assert_eq!(a - b, vector4!(-6.0, 8.0, -4.0));
    }

    #[test]
    fn should_multiply_vector_by_scalar() {
        let v = vector4!(1.0, -2.0, 3.0);
        assert_eq!(v * 0.5, vector4!(0.5, -1.0, 1.5));
    }

    #[test]
    fn should_multiply_vector_by_vector() {
        let a = vector4!(-1.0, 0.0, 2.0);
        let b = vector4!(2.0, 0.0, 4.0);
        assert_eq!(a * b, vector4!(-2.0, 0.0, 8.0));
    }

    #[test]
    fn should_divide_vector_by_scalar() {
        let v = vector4!(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, vector4!(0.5, -1.0, 1.5));
    }

    #[test]
    fn should_calculate_vector_magnitude_1() {
        let v = vector4!(1.0, 0.0, 0.0);
        assert_eq!(magnitude(v), 1.0);
    }

    #[test]
    fn should_calculate_vector_magnitude_2() {
        let v = vector4!(0.0, 1.0, 0.0);
        assert_eq!(magnitude(v), 1.0);
    }

    #[test]
    fn should_calculate_vector_magnitude_3() {
        let v = vector4!(0.0, 0.0, 1.0);
        assert_eq!(magnitude(v), 1.0);
    }

    #[test]
    fn should_calculate_vector_magnitude_4() {
        let v = vector4!(1.0, 5.0, 2.0);
        assert_eq!(magnitude(v), 30.0_f32.sqrt());
    }

    #[test]
    fn should_calculate_vector_magnitude_5() {
        let v = vector4!(-1.0, -4.0, -8.0);
        assert_eq!(magnitude(v), 9.0);
    }

    #[test]
    fn should_normalize_vector() {
        let v = vector4!(-1.0, -4.0, -8.0);
        assert_eq!(
            round_to(normalize(v),4),
            round_to(vector4!(-0.0, -0.5, -1.0), 4),
        );
    }

    #[test]
    fn should_magnitude_of_normalized_vector_1() {
        let v = vector4!(-1.0, -4.0, -8.0);
        assert_eq!(magnitude(normalize(v)), 1.0);
    }
}
