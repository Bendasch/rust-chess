extern crate glm;
pub use glm::*;


pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> glm::Mat4 {
    glm::mat4(
        2.0/(right-left), 0.0, 0.0, 0.0,
        0.0, 2.0/(top-bottom), 0.0, 0.0,
        0.0, 0.0, -2.0/(far-near), 0.0,
        -(right+left)/(right-left), -(top+bottom)/(top-bottom), -(far+near)/(far-near), 1.0
    )
}

pub fn translate(x: f32, y: f32, z: f32) -> glm::Mat4 {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -x, -y, -z, 1.0
    )    
}

pub fn rotate_x(theta: f32) -> glm::Mat4 {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, theta.cos(), -theta.sin(), 0.0,
        0.0, theta.sin(), theta.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    )    
}

pub fn rotate_z(theta: f32) -> glm::Mat4 {
    glm::mat4(
        theta.cos(), theta.sin(), 0.0, 0.0,
        -theta.sin(), theta.cos(), 0.0, 0.0, 
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )    
}