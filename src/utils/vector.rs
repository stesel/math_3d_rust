use std::ops::{Neg, Add, Sub, Mul, Div};
use crate::utils::scalar::RoundTo;

#[derive(Debug, Clone)]
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

    pub fn magnitude(v: Self) -> f32 {
        (v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w).sqrt()
    }

    pub fn normalize(v: Self) -> Self {
        v.clone() / Self::magnitude(v)
    }

    pub fn round_to(v: Self, digits_after_comma: u8) -> Self {
        Self::new(
            v.x.round_to(digits_after_comma),
            v.y.round_to(digits_after_comma),
            v.z.round_to(digits_after_comma),
            v.w.round_to(digits_after_comma)
        )
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
            0.0
        )
    }

    pub fn get(&self, index: usize) -> f32 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => { panic!("Attempt to get unacceptable index") }
        }
    }

    pub fn set(&mut self, index: usize, value: f32) -> &Self {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => { panic!("Attempt to set unacceptable index") }
        }
        self
    }
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
macro_rules! vector {
    ($x: expr, $y: expr, $z: expr) => (
        Vector4::new($x, $y, $z, 0.0)
    );
    ($x: expr, $y: expr, $z: expr, $w: expr) => (
        Vector4::new($x, $y, $z, $w)
    );
}
