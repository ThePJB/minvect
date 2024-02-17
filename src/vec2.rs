use serde::{Serialize, Deserialize};
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
pub const fn vec2(x: f32, y: f32) -> Vec2 { Vec2 { x, y } }
impl Vec2 {
    pub fn norm(&self) -> f32 { self.dot(*self).sqrt() }
    pub fn dist(&self, other: Self) -> f32 { (*self - other).norm() }
    pub fn lerp(&self, other: Self, t: f32) -> Self { *self * t + other * (1.0 - t) }
    pub fn unit(&self) -> Option<Self> { let n = self.norm(); if n == 0.0 {None} else {Some(*self/n)}}
    pub fn dot(&self, other: Self) -> f32 { self.x*other.x + self.y*other.y }
    pub fn min(&self, other: Self) -> Self { vec2(self.x.min(other.x), self.y.min(other.y)) }
    pub fn max(&self, other: Self) -> Self { vec2(self.x.max(other.x), self.y.max(other.y)) }
    pub fn floor(&self) -> Self { vec2(self.x.floor(), self.y.floor()) }
    pub fn ceil(&self) -> Self { vec2(self.x.ceil(), self.y.ceil()) }
    pub fn cross(&self, other: Vec2) -> f32 { self.x * other.y - other.x * self.y }
    pub fn mul_complex(&self, other: Vec2) -> Vec2 {
        let a = self.x;
        let b = self.y;
        let c = other.x;
        let d = other.y;
        vec2(a*c - b*d, a*d + c*b)
    }
    pub fn div_complex(&self, other: Vec2) -> Vec2 {
        let a = self.x;
        let b = self.y;
        let c = other.x;
        let d = other.y;

        let denom = c*c + d*d;

        vec2(a*c + b*d, b*c - a*d) / denom
    }
}
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        vec2(-self.x,-self.y)
    }
}
impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        vec2(self.x+rhs.x, self.y+rhs.y)
    }
}
impl std::ops::Add<Vec2> for f32 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        vec2(self + rhs.x, self + rhs.y)
    }
}
impl std::ops::Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Vec2 {
        vec2(self.x + rhs, self.y + rhs)
    }
}
impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}
impl std::ops::Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        vec2(self.x - rhs, self.y - rhs)
    }
}
impl std::ops::Sub<Vec2> for f32 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        vec2(self - rhs.x, self - rhs.y)
    }
}
impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        vec2(self.x*rhs, self.y*rhs)
    }
}
impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        vec2(self*rhs.x, self*rhs.y)
    }
}
impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        vec2(self.x*rhs.x, self.y*rhs.y)
    }
}
impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        vec2(self.x/rhs,self.y/rhs)
    }
}
impl std::ops::Div<Vec2> for f32 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Vec2 {
        vec2(self/rhs.x,self/rhs.y)
    }
}
impl std::ops::Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Vec2 {
        vec2(self.x/rhs.x,self.y/rhs.y)
    }
}
impl std::ops::Rem<f32> for Vec2 {
    type Output = Vec2;
    fn rem(self, rhs: f32) -> Self::Output {
        vec2(self.x % rhs, self.y % rhs)
    }
}
impl std::ops::Rem<Vec2> for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: Vec2) -> Self::Output {
        vec2(self.x % rhs.x, self.y % rhs.y)
    }
}
impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
impl std::ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
impl std::ops::RemAssign<f32> for Vec2 {
    fn rem_assign(&mut self, rhs: f32) {
        *self = vec2(self.x % rhs, self.y % rhs);
    }
}