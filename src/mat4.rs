use crate::*;

/// mat4 multiplication
/// actually faster than loops version? who knows
pub fn mat4_mul(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    [
        a[0]*b[0] + a[1]*b[4] + a[2]*b[8] + a[3]*b[12],
        a[0]*b[1] + a[1]*b[5] + a[2]*b[9] + a[3]*b[13],
        a[0]*b[2] + a[1]*b[6] + a[2]*b[10] + a[3]*b[14],
        a[0]*b[3] + a[1]*b[7] + a[2]*b[11] + a[3]*b[15],

        a[4]*b[0] + a[5]*b[4] + a[6]*b[8] + a[7]*b[12],
        a[4]*b[1] + a[5]*b[5] + a[6]*b[9] + a[7]*b[13],
        a[4]*b[2] + a[5]*b[6] + a[6]*b[10] + a[7]*b[14],
        a[4]*b[3] + a[5]*b[7] + a[6]*b[11] + a[7]*b[15],

        a[8]*b[0] + a[9]*b[4] + a[10]*b[8] + a[11]*b[12],
        a[8]*b[1] + a[9]*b[5] + a[10]*b[9] + a[11]*b[13],
        a[8]*b[2] + a[9]*b[6] + a[10]*b[10] + a[11]*b[14],
        a[8]*b[3] + a[9]*b[7] + a[10]*b[11] + a[11]*b[15],

        a[12]*b[0] + a[13]*b[4] + a[14]*b[8] + a[15]*b[12],
        a[12]*b[1] + a[13]*b[5] + a[14]*b[9] + a[15]*b[13],
        a[12]*b[2] + a[13]*b[6] + a[14]*b[10] + a[15]*b[14],
        a[12]*b[3] + a[13]*b[7] + a[14]*b[11] + a[15]*b[15],
    ]
}

/// mat4 view matrix for camera
pub fn mat4_view(pos: Vec3, dir: Vec3) -> [f32; 16] {
    let up = vec3(0.0, 1.0, 0.0);

    let zaxis = dir;
    let xaxis = up.cross(zaxis).unit().unwrap();
    let yaxis = zaxis.cross(xaxis).unit().unwrap();
    [
        xaxis.x, yaxis.x, zaxis.x, 0.0,
        xaxis.y, yaxis.y, zaxis.y, 0.0,
        xaxis.z, yaxis.z, zaxis.z, 0.0,
        pos.dot(xaxis), pos.dot(yaxis), pos.dot(zaxis), 1.0,
    ]
}

/// mat4 projection: fov in radians
fn mat4_proj(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> [f32; 16] {
    let tan_half_fov = (fov / 2.0).tan();
    let z_range = z_near - z_far;

    [
        1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_half_fov, 0.0, 0.0,
        0.0, 0.0, (z_near + z_far) / z_range, 2.0 * z_far * z_near / z_range,
        0.0, 0.0, -1.0, 0.0,
    ]
}

/// mat4 transpose
pub fn mat4_transpose(a: &[f32; 16]) -> [f32; 16] {
    [
        a[0], a[4], a[8], a[12],
        a[1], a[5], a[9], a[13],
        a[2], a[6], a[10], a[14],
        a[3], a[7], a[11], a[15],
    ]
}

/// camera view * projection matrix
pub fn cam_vp(cam_pos: Vec3, cam_dir: Vec3, fov: f32, aspect: f32, z_near: f32, z_far: f32) -> [f32; 16] {
    let view_matrix = mat4_view(cam_pos, cam_dir);
    let view_matrix = mat4_transpose(&view_matrix);
    let projection_matrix: [f32; 16] = mat4_proj(fov, aspect, z_near, z_far);
    mat4_mul(&projection_matrix, &view_matrix)
}

/// homogeneous transformation of a vec3 with a mat4
pub fn mat4_trans_homog(v: Vec3, mat: &[f32; 16]) -> Vec3 {
    let x = v.x * mat[0] + v.y * mat[4] + v.z * mat[8] + mat[12];
    let y = v.x * mat[1] + v.y * mat[5] + v.z * mat[9] + mat[13];
    let z = v.x * mat[2] + v.y * mat[6] + v.z * mat[10] + mat[14];
    vec3(x, y, z)
}