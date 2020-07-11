pub trait RoundTo<T> {
    fn round_to(&self, digits_after_comma: u8) -> T;
}

impl RoundTo<f32> for f32 {
    fn round_to(&self, digits_after_comma: u8) -> Self {
        let multiplier = 10_i32.pow(digits_after_comma.into()) as f32;
        (self * multiplier).round() / multiplier
    }
}
