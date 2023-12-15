use serde::{Serialize, Deserialize};
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }
impl Vec3 {
    pub fn norm(&self) -> f32 { self.dot(*self).sqrt() }
    pub fn dist(&self, other: Self) -> f32 { (*self - other).norm() }
    pub fn lerp(&self, other: Self, t: f32) -> Self { *self * t + other * (1.0 - t) }
    pub fn unit(&self) -> Option<Self> { let n = self.norm(); if n == 0.0 {None} else {Some(*self/n)}}
    pub fn dot(&self, other: Self) -> f32 { self.x * other.x + self.y * other.y + self.z * other.z }
    pub fn min(&self, other: Self) -> Self { vec3(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z)) }
    pub fn max(&self, other: Self) -> Self { vec3(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z)) }
    pub fn floor(&self) -> Self { vec3(self.x.floor(), self.y.floor(), self.z.floor()) }
    pub fn ceil(&self) -> Self { vec3(self.x.ceil(), self.y.ceil(), self.z.ceil()) }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
impl std::ops::Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl std::ops::Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}
impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
impl std::ops::RemAssign<f32> for Vec3 {
    fn rem_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        };
    }
}
impl std::ops::RemAssign<Vec3> for Vec3 {
    fn rem_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        };
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("[{:.*}, {:.*}, {:.*}]", decimals, self.x, decimals, self.y, decimals, self.z);
        f.pad_integral(true, "", &string)
    }
}