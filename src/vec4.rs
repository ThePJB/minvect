use crate::util::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4 { x, y, z, w } }

impl Vec4 {
    pub fn dot(&self, other: Vec4) -> f32 {
        self.x*other.x + self.y * other.y + self.z*other.z + self.w*other.w
    }
    pub fn lerp(&self, other: Vec4, t: f32) -> Vec4 { vec4(lerp(self.x, other.x, t), lerp(self.y, other.y, t), lerp(self.z, other.z, t), lerp(self.w, other.w, t)) }

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
