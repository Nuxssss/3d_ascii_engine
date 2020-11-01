use std::ops::{Add, Mul, Sub};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn abs(self) -> Point3D {
        Point3D {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }
}

impl Sub for Point3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Point3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
