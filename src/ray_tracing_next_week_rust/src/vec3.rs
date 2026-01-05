use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Neg;
use std::f64;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        return Vec3::new(self.x / length, self.y / length, self.z / length);
    }

    pub fn near_zero(&self) -> bool {
        return self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8;
    }

    pub fn component(&self, n: u16) -> f64 {
        if n == 0 {
            return self.x;
        }
        if n == 1 {
            return self.y;
        }
        return self.z;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z;
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    return Vec3::new(
        lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.z * rhs.x - lhs.x * rhs.z,
        lhs.x * rhs.y - lhs.y * rhs.x
    );
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{:.3} y:{:.3} z:{:.3}", self.x, self.y, self.z)
    }
}