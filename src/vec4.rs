use serde::{Serialize, Deserialize};
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}
impl Vec4 {
    pub fn norm(&self) -> f32 { self.dot(*self).sqrt() }
    pub fn dist(&self, other: Self) -> f32 { (*self - other).norm() }
    pub fn lerp(&self, other: Self, t: f32) -> Self { *self * t + other * (1.0 - t) }
    pub fn unit(&self) -> Option<Self> { let n = self.norm(); if n == 0.0 {None} else {Some(*self/n)}}
    pub fn dot(&self, other: Self) -> f32 { self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w }
    pub fn floor(&self) -> Self { vec4(self.x.floor(), self.y.floor(), self.z.floor(), self.w.floor()) }
    pub fn ceil(&self) -> Self { vec4(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil()) }
    pub fn hsv_to_rgb(&self) -> Vec4 {
        let v = self.z;
        let hh = (self.x % 360.0) / 60.0;
        let i = hh.floor() as i32;
        let ff = hh - i as f32;
        let p = self.z * (1.0 - self.y);
        let q = self.z * (1.0 - self.y * ff);
        let t = self.z * (1.0 - self.y * (1.0 - ff));
        match i {
            0 => vec4(v, t, p, self.w),
            1 => vec4(q, v, p, self.w),
            2 => vec4(p, v, t, self.w),
            3 => vec4(p, q, v, self.w),
            4 => vec4(t, p, v, self.w),
            5 => vec4(v, p, q, self.w),
            _ => panic!("unreachable"),
        }
    }
}
impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl std::ops::Add<f32> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: f32) -> Vec4 {
        Vec4 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}
impl std::ops::Add<Vec4> for f32 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: rhs.x + self,
            y: rhs.y + self,
            z: rhs.z + self,
            w: rhs.w + self,
        }
    }
}
impl std::ops::Sub<f32> for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: f32) -> Vec4 {
        Vec4 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}
impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl std::ops::Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f32) -> Vec4 {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl std::ops::Mul<Vec4> for f32 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        rhs * self
    }
}
impl std::ops::Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f32) -> Vec4 {
        Vec4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl std::ops::Div<Vec4> for f32 {
    type Output = Vec4;
    fn div(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
            w: self / rhs.w,
        }
    }
}
impl std::fmt::Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!(
            "[{:.*}, {:.*}, {:.*}, {:.*}]",
            decimals, self.x, decimals, self.y, decimals, self.z, decimals, self.w
        );
        f.pad_integral(true, "", &string)
    }
}