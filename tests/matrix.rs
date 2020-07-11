#[macro_use]
extern crate math_3d_rust;
use math_3d_rust::utils::matrix::{Matrix};
use math_3d_rust::utils::vector::{Vector4};

#[cfg(test)]
mod matrix {
    use super::{Matrix, Vector4};

    #[test]
    fn should_create_vector() {
        let m = matrix!(4, 4);
        assert_eq!(m.len(), 16);
    }

    #[test]
    fn should_create_identity_vector() {
        let m = identity_matrix!(4, 4);
        assert_eq!(m.len(), 16);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(3, 3), 1.0);
    }

    #[test]
    fn should_multiply_two_4x4_matrices() {
        let a = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        );
        let b = Matrix::fill(
            matrix!(4, 4),
            vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]
        );
        let output = Matrix::fill(
            matrix!(4, 4),
            vec![-34.0, 44.0, -54.0, 64.0, -82.0, 108.0, -134.0, 160.0, -34.0, 44.0, -54.0, 64.0, -82.0, 108.0, -134.0, 160.0, 4.0, 4.0]
        );
        assert_eq!(a * b, output);
    }

    #[test]
    fn should_multiply_matrix_by_vector() {
        let m = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.0]
        );
        let v = vector!(1.0, 2.0, 3.0, 1.0);
        assert_eq!(m * v, vector!(18.0, 46.0, 18.0, 1.0));
    }

    #[test]
    fn should_multiply_matrix_by_identity_matrix() {
        let m = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.0]
        );
        let im = identity_matrix!(4, 4);
        let output = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(m * im, output);
    }

    #[test]
    fn should_multiply_identity_matrix_by_vector() {
        let m = identity_matrix!(4, 4);
        let v = vector!(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m * v.clone(), v);
    }

    #[test]
    fn should_multiply_matrix_with_inverse() {
        let a = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        );
        let b = Matrix::fill(
            matrix!(4, 4),
            vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, -1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]
        );
        let c = a.clone() * b.clone();
        let result = c * Matrix::inverse(b);
        assert_eq!(result, a);
    }

    #[test]
    fn should_create_submatrix_from_3x3() {
        let m = Matrix::fill(
            matrix!(3, 3),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
        let output = Matrix::fill(
            matrix!(2, 2),
            vec![4.0, 5.0, 7.0, 8.0, 2.0, 2.0]
        );
        let result = Matrix::submatrix(m, 0, 2);
        assert_eq!(result, output);
    }

    #[test]
    fn should_create_submatrix_from_4x4() {
        let m = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.0]
        );
        let output = Matrix::fill(
            matrix!(3, 3),
            vec![5.0, 6.0, 8.0, 1.0, 2.0, 4.0, 0.0, 0.0, 1.0, 3.0, 3.0]
        );
        let result = Matrix::submatrix(m, 0, 2);
        assert_eq!(result, output);
    }

    #[test]
    fn should_calculate_minor_for_3x3_matrix() {
        let m = Matrix::fill(
            matrix!(3, 3),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
        let sub_matrix = Matrix::submatrix(m.clone(), 1, 0);
        let minor = Matrix::minor(m, 1, 0);
        let sub_matrix_determinant = Matrix::determinant(sub_matrix);
        assert_eq!(sub_matrix_determinant, -6.0);
        assert_eq!(sub_matrix_determinant, minor);
    }

    #[test]
    fn should_calcualte_cofactor_for_3x3_matrix() {
        let m = Matrix::fill(
            matrix!(3, 3),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
        assert_eq!(Matrix::minor(m.clone(), 0, 0), -3.0);
        assert_eq!(Matrix::cofactor(m.clone(), 0, 0), -3.0);
        assert_eq!(Matrix::minor(m.clone(), 1, 0), -6.0);
        assert_eq!(Matrix::cofactor(m, 1, 0), 6.0);
    }

    #[test]
    fn should_calculate_inverse_of_the_matrix() {
        let m = Matrix::fill(
            matrix!(4, 4),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, -1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]
        );
        let im = Matrix::fill(
            matrix!(4, 4),
            vec![-0.4375, 0.1875, 0.4375, -0.1875, -0.5, 0.25, -0.5, 0.25, 0.3125, -0.0625, -0.3125, 0.0625, 0.375, -0.125, 0.375, -0.125, 4.0, 4.0]
        );
        assert_eq!(Matrix::round_to(Matrix::inverse(m), 4), im);
    }

    #[test]
    fn should_calculate_inverse_of_the_identity_matrix() {
        let m = identity_matrix!(4, 4);
        assert_eq!(Matrix::inverse(m.clone()), m);
    }
}
