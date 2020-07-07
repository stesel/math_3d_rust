use std::ops::Neg;

#[derive(Debug)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vector4 {x, y, z, w}
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

#[macro_export]
macro_rules! vector {
    ($x: expr, $y: expr, $z: expr) => (
        Vector4::new($x, $y, $z, 0.0)
    );
    ($x: expr, $y: expr, $z: expr, $w: expr) => (
        Vector4::new($x, $y, $z, $w)
    );
}
