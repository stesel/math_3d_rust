pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z:f64, w: f64) -> Self {
        return Vector4 {
            x,
            y,
            z,
            w,
        }
    }
}

