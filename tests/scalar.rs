extern crate math_3d_rust;
use math_3d_rust::utils::scalar::RoundTo;

#[cfg(test)]
mod scalar {
    use super::{RoundTo};

    #[test]
    fn should_round_float_number_to_digits_after_comma() {
        assert_eq!(0.0.round_to(1), 0.0);
        assert_eq!(1.11111.round_to(3), 1.111);
        assert_eq!(3.0845689.round_to(5), 3.08457);
    }
}
