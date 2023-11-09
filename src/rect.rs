use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
pub struct Rect {
    pub xy: Vec2,
    pub wh: Vec2,
}
/// rect from scalars
pub const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { xy: vec2(x,y), wh: vec2(w,h)}
}
/// rect from vectors
pub const fn rectv(xy: Vec2, wh: Vec2) -> Rect {
    Rect { xy, wh }
}
/// rect centered
pub fn rectc(xy: Vec2, wh: Vec2) -> Rect {
    Rect { xy: xy - wh/2.0, wh }
}

impl Rect {
    pub fn i(&self) -> Vec2 { vec2(self.wh.x, 0.0) }
    pub fn j(&self) -> Vec2 { vec2(0.0, self.wh.y) }
    pub fn tl(&self) -> Vec2 { self.xy }
    pub fn tr(&self) -> Vec2 { self.xy + self.i() }
    pub fn bl(&self) -> Vec2 { self.xy + self.j() }
    pub fn br(&self) -> Vec2 { self.xy + self.wh }
    pub fn contains(&self, p: Vec2) -> bool {
        let p = p - self.xy;
        p.x >= 0.0 && p.x <= self.wh.x &&
        p.y >= 0.0 && p.y <= self.wh.y
    }
    // p uv not ndc
    pub fn rect_to_world(&self, p: Vec2) -> Vec2 {
        self.xy + p * self.wh
    }
    pub fn world_to_rect(&self, p: Vec2) -> Vec2 {
        (p - self.xy) / self.wh
    }
    pub fn uniform_grid(&self, i: usize, j: usize, w: usize, h: usize) -> Rect {
        let v = self.wh / vec2(w as f32, h as f32);
        rectv(self.xy + vec2(i as f32, j as f32) * v, v)
    }
    /// returns grid rect i,j. widths and heights as weights of parent vect.
    pub fn grid(&self, i: usize, j: usize, widths: &[f32], heights: &[f32]) -> Rect {
        let mut acc = vec2(0.0, 0.0);
        let sw = widths.iter().fold(0.0f32, |acc: f32, e| acc + e);
        let sh = widths.iter().fold(0.0f32, |acc: f32, e| acc + e);
        for x in 0..i {
            acc.x += widths[x];
        }
        for y in 0..j {
            acc.y += heights[y];
        }
        acc.x /= sw;
        acc.y /= sh;
        acc *= self.wh;

        acc += self.xy;
        
        let wh = vec2(widths[i]/sw, heights[j]/sh) * self.wh;
        rectv(acc, wh)
    }
    pub fn fit_aspect(&self, a: f32) -> Rect {
        let a_self = self.wh.x/self.wh.y;
        if a_self > a {
            // parent wider
            rect((self.wh.x - self.wh.x*(1.0/a))/2.0, 0.0, self.wh.x*1.0/a, self.wh.y)
        } else {
            // child wider
            rect(0.0, (self.wh.y - self.wh.y*(1.0/a))/2.0, self.wh.x, self.wh.y/a)
        }
    }

}

#[test]
fn test_rect_rtw() {
    let r = rect(-2.0, -2.0, 4.0, 4.0);
    let v = vec2(0.25, 0.25);
    let p = r.rect_to_world(v);
    assert_eq!(p, vec2(-1.0, -1.0));
}

#[test]
fn test_rect_wtr() {
    let r = rect(4.0, 4.0, 1.0, 2.0);
    let v = vec2(4.5, 5.0);
    let p = r.world_to_rect(v);
    assert_eq!(p, vec2(0.5, 0.5));
}

#[test]
fn test_rect_contains() {
    let r = rect(1.0, 1.0, 2.0, 2.0);
    
    // Test a point inside the rectangle
    let inside_point = vec2(1.5, 1.5);
    assert!(r.contains(inside_point));
    
    // Test a point outside the rectangle
    let outside_point = vec2(3.5, 3.5);
    assert!(!r.contains(outside_point));
}

#[test]
fn test_rect_uniform_grid() {
    let r = rect(0.0, 0.0, 4.0, 4.0);
    let i = 1;
    let j = 0;
    let w = 2;
    let h = 2;
    let grid_rect = r.uniform_grid(i, j, w, h);
    assert_eq!(grid_rect, rect(2.0, 0.0, 2.0, 2.0));
}

#[test]
fn test_rect_grid() {
    let r = rect(0.0, 0.0, 4.0, 4.0);
    let i = 1;
    let j = 1;
    let widths = vec![1.0, 2.0, 1.0];
    let heights = vec![0.5, 2.5, 1.0];
    let grid_rect = r.grid(i, j, &widths, &heights);
    assert_eq!(grid_rect, rect(1.0, 0.5, 2.0, 5.0 / 8.0 * 4.0));
}

#[test]
fn test_rect_fit_aspect() {
    let r = rect(0.0, 0.0, 4.0, 4.0);
    let aspect = 2.0;
    let fitted_rect = r.fit_aspect(aspect);
    assert_eq!(fitted_rect, rect(0.0, 1.0, 4.0, 2.0));
}
