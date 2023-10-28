use crate::*;

pub fn mat3_inv(mat: &[f32; 9]) -> [f32; 9] {
    let a = mat[0];
    let b = mat[1];
    let c = mat[2];
    let d = mat[3];
    let e = mat[4];
    let f = mat[5];
    let g = mat[6];
    let h = mat[7];
    let i = mat[8];

    let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    
    if det == 0.0 {
        // The matrix is singular, return an identity matrix as a placeholder
        return [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    }

    let inv_det = 1.0 / det;

    let result = [
        (e * i - f * h) * inv_det,
        (c * h - b * i) * inv_det,
        (b * f - c * e) * inv_det,
        (f * g - d * i) * inv_det,
        (a * i - c * g) * inv_det,
        (c * d - a * f) * inv_det,
        (d * h - e * g) * inv_det,
        (b * g - a * h) * inv_det,
        (a * e - b * d) * inv_det,
    ];

    result
}


pub fn mat3_trans_homog(p: Vec2, m: &[f32; 9]) -> Vec2 {
    let x = p.x * m[0] + p.y * m[1] + m[2];
    let y = p.x * m[3] + p.y * m[4] + m[5];
    vec2(x, y)    
}
