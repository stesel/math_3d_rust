use std::ops::{Neg, Add, Sub, Mul, Div};

pub trait RoundTo<T> {
    fn round_to(&self, digits_after_comma: u8) -> T;
}

impl RoundTo<f32> for f32 {
    fn round_to(&self, digits_after_comma: u8) -> Self {
        (self * digits_after_comma as f32).round() / digits_after_comma as f32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 {x, y, z, w}
    }
}

pub fn magnitude(v: Vector4) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w).sqrt()
}

pub fn normalize(v: Vector4) -> Vector4 {
    v / magnitude(v)
}

pub fn round_to(v: Vector4, digits_after_comma: u8) -> Vector4 {
    Vector4::new(
        v.x.round_to(digits_after_comma),
        v.y.round_to(digits_after_comma),
        v.z.round_to(digits_after_comma),
        v.w.round_to(digits_after_comma),
    )
}

impl PartialEq for Vector4 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z && self.w == rhs.w
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4::new(self.x * -1.0, self.y * -1.0, self.z * -1.0, self.w * -1.0)
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector4::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl Mul for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector4::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, scalar: f32) -> Self::Output {
        Vector4::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }
}

#[macro_export]
macro_rules! vector4 {
    ($x: expr, $y: expr, $z: expr) => (
        Vector4::new($x, $y, $z, 0.0)
    );
    ($x: expr, $y: expr, $z: expr, $w: expr) => (
        Vector4::new($x, $y, $z, $w)
    );
}
