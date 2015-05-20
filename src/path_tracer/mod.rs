use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug,Clone,Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3d {
    pub fn normalized(self) -> Vec3d {
        self / self.dot(self).sqrt()
    }
    pub fn dot(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }
    pub fn zero() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn cross(self, other: Vec3d) -> Vec3d {
        Vec3d { 
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
    pub fn max_component(self) -> f64 {
        if self.x > self.y && self.x > self.z { self.x }
        else if self.y > self.x && self.y > self.z { self.y }
        else { self.z }
    }
    pub fn neg(self) -> Vec3d {
        Vec3d { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: f64) -> Vec3d {
        Vec3d { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, other: f64) -> Vec3d {
        Vec3d { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}
