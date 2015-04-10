use std::mem;

pub use vecmath::{
    Vector3,
    Matrix4,
    row_mat4_mul,
    row_mat4_transform,
    mat4_id,
};

pub use quaternion::Quaternion;

pub fn lerp_quaternion(q1: &Quaternion<f32>, q2: &Quaternion<f32>, blend_factor: &f32) -> Quaternion<f32> {

    let dot = q1.0 * q2.0 + q1.1[0] * q2.1[0] + q1.1[1] * q2.1[1] + q1.1[2] * q2.1[2];

    let s = 1.0 - blend_factor;
    let t: f32 = if dot > 0.0 { *blend_factor } else { -blend_factor };

    let w = s * q1.0 + t * q2.0;
    let x = s * q1.1[0] + t * q2.1[0];
    let y = s * q1.1[1] + t * q2.1[1];
    let z = s * q1.1[2] + t * q2.1[2];

    let inv_sqrt_len = inv_sqrt(w * w + x * x + y * y + z * z);
    (w * inv_sqrt_len, [x  * inv_sqrt_len, y  * inv_sqrt_len, z  * inv_sqrt_len])
}

/// rotation matrix for `a` radians about z
pub fn mat4_rotate_z(a: f32) -> Matrix4<f32> {
    [
        [a.cos(), -a.sin(), 0.0, 0.0],
        [a.sin(), a.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn matrix_to_quaternion(m: &Matrix4<f32>) -> Quaternion<f32> {

    let mut q = [0.0, 0.0, 0.0, 0.0];

    let next = [1, 2, 0];

    let trace = m[0][0] + m[1][1] + m[2][2];

    if trace > 0.0 {

        let t = trace + 1.0;
        let s = inv_sqrt(t) * 0.5;

        q[3] = s * t;
        q[0] = (m[1][2] - m[2][1]) * s;
        q[1] = (m[2][0] - m[0][2]) * s;
        q[2] = (m[0][1] - m[1][0]) * s;

    } else {

        let mut i = 0;

        if m[1][1] > m[0][0] {
            i = 1;
        }

        if m[2][2] > m[i][i] {
            i = 2;
        }

        let j = next[i];
        let k = next[j];

        let t = (m[i][i] - (m[j][j] + m[k][k])) + 1.0;
        let s = inv_sqrt(t) * 0.5;

        q[i] = s * t;
        q[3] = (m[j][k] - m[k][j]) * s;
        q[j] = (m[i][j] + m[j][i]) * s;
        q[k] = (m[i][k] + m[k][i]) * s;

    }

    (q[3], [q[0], q[1], q[2]])
}

///
/// See http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/
///
pub fn quaternion_to_matrix(q: Quaternion<f32>) -> Matrix4<f32> {

    let w = q.0;
    let x = q.1[0];
    let y = q.1[1];
    let z = q.1[2];

    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;

    let xx2 = x2 * x;
    let xy2 = x2 * y;
    let xz2 = x2 * z;

    let yy2 = y2 * y;
    let yz2 = y2 * z;
    let zz2 = z2 * z;

    let wy2 = y2 * w;
    let wz2 = z2 * w;
    let wx2 = x2 * w;

    [
        [1.0 - yy2 - zz2, xy2 + wz2, xz2 - wy2, 0.0],
        [xy2 - wz2, 1.0 - xx2 - zz2, yz2 + wx2, 0.0],
        [xz2 + wy2, yz2 - wx2, 1.0 - xx2 - yy2, 0.0],
        [0.0, 0.0,  0.0,  1.0]
    ]

}

pub fn inv_sqrt(x: f32) -> f32 {

    let x2: f32 = x * 0.5;
    let mut y: f32 = x;

    let mut i: i32 = unsafe { mem::transmute(y) };
    i = 0x5f3759df - (i >> 1);
    y = unsafe { mem::transmute(i) };

    y = y * (1.5 - (x2 * y * y));
    y

}

