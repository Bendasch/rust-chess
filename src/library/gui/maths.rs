extern crate glm;
pub use glm::{mat4, vec4, GenMat, GenSquareMat, Mat4, Vec4};

#[rustfmt::skip]
pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> glm::Mat4 {
    glm::mat4(
        2.0/(r-l),      0.0,            0.0,            0.0,   
        0.0,            2.0/(t-b),      0.0,            0.0,  
        0.0,            0.0,            -2.0/(f-n),     0.0,
        -(r+l)/(r-l),  -(t+b)/(t-b),   -(f+n)/(f-n),    1.0,
    )
}

#[rustfmt::skip]
pub fn translate(x: f32, y: f32, z: f32) -> glm::Mat4 {
    glm::mat4(
        1.0,    0.0,    0.0,    0.0, 
        0.0,    1.0,    0.0,    0.0, 
        0.0,    0.0,    1.0,    0.0, 
        x,      y,      z,      1.0,
    )
}

#[rustfmt::skip]
pub fn scale(x: f32, y: f32, z:f32) -> glm::Mat4 {
    glm::mat4(
        x,      0.0,    0.0,    0.0,
        0.0,    y,      0.0,    0.0,
        0.0,    0.0,    z,      0.0,
        0.0,    0.0,    0.0,    1.0
    )
}

#[rustfmt::skip]
pub fn rotate_x(th: f32) -> glm::Mat4 {
    glm::mat4(
        1.0,    0.0,        0.0,        0.0,
        0.0,    th.cos(),   -th.sin(),  0.0,   
        0.0,    th.sin(),   th.cos(),   0.0,   
        0.0,    0.0,        0.0,        1.0,
    )
}

#[rustfmt::skip]
pub fn rotate_z(alpha: f32) -> glm::Mat4 {
    let theta = deg_to_radians(alpha);
    glm::mat4(
        theta.cos(),    theta.sin(),    0.0,    0.0,
        -theta.sin(),   theta.cos(),    0.0,    0.0,
        0.0,            0.0,            1.0,    0.0,
        0.0,            0.0,            0.0,    1.0,
    )
}

fn deg_to_radians(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

/*
pub fn transpose(mat: &Mat4) -> Mat4 {
    mat.transpose()
}
*/
