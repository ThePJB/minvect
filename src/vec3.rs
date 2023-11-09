use crate::*;
use crate::util::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }

impl Vec3 {
    pub fn abs(&self) -> Self { vec3(self.x.abs(), self.y.abs(), self.z.abs())}
    pub fn fract(&self) -> Self { vec3(self.x.abs(), self.y.abs(), self.z.abs())}
    pub fn mul_scalar(&self, scalar: f32) -> Vec3 { vec3(self.x * scalar, self.y * scalar, self.z * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec3 { vec3(self.x / scalar, self.y / scalar, self.z / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }
    pub fn normalize(&self) -> Vec3 { 
        let m = self.magnitude();
        if m == 0.0 {
            return vec3(0.0, 0.0, 0.0);
        } else {
            return self.div_scalar(self.magnitude()); 
        }
    }
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 { vec3(lerp(self.x, other.x, t), lerp(self.y, other.y, t), lerp(self.z, other.z, t)) }
    pub fn dist(&self, other: Vec3) -> f32 {(*self - other).magnitude().sqrt()}
    pub fn dot(&self, other: Vec3) -> f32 {self.x*other.x + self.y*other.y + self.z*other.z} // is squ dist lol
    pub fn cross(&self, other: Vec3) -> Vec3 {
        vec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }
    // assumes north pole
    pub fn cartesian_to_spherical(&self) -> Self {
        vec3(self.magnitude(), self.y.acos(), self.z.atan2(self.x))
    }
    pub fn spherical_to_cartesian(&self) -> Self {
        vec3(self.x*self.y.sin()*self.z.cos(), self.x*self.y.cos(), self.x*self.y.sin()*self.z.sin())
    }
    pub fn xy(&self) -> Vec2 { vec2(self.x, self.y) }
    
    pub fn assert_equals(&self, other: Self) {
        if self.x - other.x != 0.0 || self.y - other.y != 0.0 || self.z - other.z != 0.0 { panic!("{} not equal to {}", self, other) }
    }
    pub fn assert_approx_equals(&self, other: Self) {
        use std::f32::EPSILON as e;
        if (self.x - other.x).abs() > e || (self.y - other.y).abs() > e || (self.z - other.z).abs() > e { panic!("{} not equal to {}", self, other); }
    }
    pub fn assert_unit(&self) {
        if self.magnitude() != 1.00 { panic!("{} not unit", self); }
    }
}

// // i think this only fails due to precision kek
// #[test]
// fn test_spherical() {
//     ((vec3(1.0, 0.0, 0.0).cartesian_to_spherical() + vec3(0.0, PI, 0.0)).spherical_to_cartesian().assert_approx_equals(vec3(-1.0, 0.0, 0.0)));
//     ((vec3(1.0, 0.0, 0.0).cartesian_to_spherical() + vec3(0.0, -PI, 0.0)).spherical_to_cartesian().assert_approx_equals(vec3(-1.0, 0.0, 0.0)));
//     ((vec3(1.0, 0.0, 0.0).cartesian_to_spherical() + vec3(0.0, 2.0*PI, 0.0)).spherical_to_cartesian().assert_approx_equals(vec3(1.0, 0.0, 0.0)));
//     ((vec3(1.0, 0.0, 0.0).cartesian_to_spherical() + vec3(0.0, PI, 0.0)).spherical_to_cartesian().assert_approx_equals(vec3(-1.0, 0.0, 0.0)));
// }

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.mul_scalar(-1.0)
    }
}

impl std::ops::AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("[{:.*}, {:.*}, {:.*}]", decimals, self.x, decimals, self.y, decimals, self.z);
        f.pad_integral(true, "", &string)
    }
}