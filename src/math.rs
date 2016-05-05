use rand::{Rng, XorShiftRng};
use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug,Clone,Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub trait Clamp {
    fn clamp(self) -> Self;
}

impl Clamp for f64 {
    #[inline]
    fn clamp(self) -> f64 {
        if self < 0.0 { 0.0 } else if self > 1.0 { 1.0 } else { self }
    }
}

impl Vec3d {
    #[inline]
    pub fn normalized(self) -> Vec3d {
        self / self.dot(self).sqrt()
    }
    #[inline]
    pub fn dot(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline]
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[inline]
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }
    #[inline]
    pub fn zero() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }
    #[inline]
    pub fn one() -> Vec3d {
        Vec3d { x: 1.0, y: 1.0, z: 1.0 }
    }
    #[inline]
    pub fn cross(self, other: Vec3d) -> Vec3d {
        Vec3d { 
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
    #[inline]
    pub fn max_component(self) -> f64 {
        if self.x > self.y && self.x > self.z { self.x }
        else if self.y > self.x && self.y > self.z { self.y }
        else { self.z }
    }
    #[inline]
    pub fn max_ordinal(self) -> u8 {
        if self.x > self.y && self.x > self.z { 0 }
        else if self.y > self.x && self.y > self.z { 1 }
        else { 2 }
    }
    #[inline]
    pub fn min_component(self) -> f64 {
        if self.x < self.y && self.x < self.z { self.x }
        else if self.y < self.x && self.y < self.z { self.y }
        else { self.z }
    }
    #[inline]
    pub fn min_ordinal(self) -> u8 {
        if self.x < self.y && self.x < self.z { 0 }
        else if self.y < self.x && self.y < self.z { 1 }
        else { 2 }
    }
    #[inline]
    pub fn abs(self) -> Vec3d {
        Vec3d { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }
    #[inline]
    pub fn neg(self) -> Vec3d {
        Vec3d { x: -self.x, y: -self.y, z: -self.z }
    }
    #[inline]
    pub fn clamp(self) -> Vec3d {
        Vec3d { x: self.x.clamp(), y: self.y.clamp(), z: self.z.clamp() }
    }
    #[inline]
    pub fn min(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x.min(other.x), y: self.y.min(other.y), z: self.z.min(other.z) }
    }
    #[inline]
    pub fn max(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x.max(other.x), y: self.y.max(other.y), z: self.z.max(other.z) }
    }
}


impl Add for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f64> for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn mul(self, other: f64) -> Vec3d {
        Vec3d { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn div(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x / other.x, y: self.y / other.y, z: self.z * other.z }
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn div(self, other: f64) -> Vec3d {
        let recip = 1.0 / other;
        Vec3d { x: self.x * recip, y: self.y * recip, z: self.z * recip }
    }
}

pub trait F64Rng {
    fn next(&mut self) -> f64;
}

impl F64Rng for XorShiftRng {
    fn next(&mut self) -> f64 {
        return self.gen::<f64>();
    }
}

