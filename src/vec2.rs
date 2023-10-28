use crate::util::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
pub fn vec2(x: f32, y: f32) -> Vec2 { Vec2 { x, y } }

impl Vec2 {
    pub fn normalize(&self) -> Vec2 { *self / self.dot(*self) }
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 { vec2(lerp(self.x, other.x, t), lerp(self.y, other.y, t)) }
    pub fn dot(&self, other: Vec2) -> f32 { self.x*other.x + self.y*other.y } 
    pub fn cross(&self, other: Vec2) -> f32 { self.x * other.y - other.x * self.y }
    pub fn from_polar(r: f32, theta: f32) -> Vec2 { r * vec2(theta.cos(), theta.sin()) }
    pub fn mul_scalar(&self, scalar: f32) -> Vec2 { vec2(self.x * scalar, self.y * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec2 { vec2(self.x / scalar, self.y / scalar) }
    pub fn mul_elem(&self, other: Vec2) -> Vec2 { vec2(self.x * other.x, self.y * other.y)}
    pub fn div_elem(&self, other: Vec2) -> Vec2 { vec2(self.x / other.x, self.y / other.y)}
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
impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
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
impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}
impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        self.mul_scalar(_rhs)
    }
}
impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        _rhs.mul_scalar(self)
    }
}
impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: f32) -> Vec2 {
        self.div_scalar(_rhs)
    }
}
impl std::ops::Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: Vec2) -> Vec2 {
        self.div_elem(_rhs)
    }
}
impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        self.mul_elem(_rhs)
    }
}
impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        self.mul_scalar(-1.0)
    }
}
impl std::ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}