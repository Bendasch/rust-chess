use crate::gl;
use crate::library::gui::{
    glfw::*,
    opengl::*,
    index_buffer::*,
    vertex_array::*,
    shader::*,
    utils::*,
};
use std::{
    ffi::{CString},
    ptr::{null_mut},
};
use libc::{c_void};

pub static WIDTH: f32 = 1024.0;
pub static HEIGHT: f32 = 768.0;

pub struct Renderer{
    pub gl: GL,
    pub window: *mut GLFWwindow
}

impl Renderer{
    
    pub unsafe fn new() -> Renderer {
        
        let window: *mut GLFWwindow;
        let monitor: *mut GLFWmonitor = null_mut();
        let share: *mut GLFWwindow = null_mut();
        
        if glfwInit() == 0 {
            panic!("Failed to initialize GLFW!");
        }
        
        let title = CString::new("Rust chess (OpenGL)").unwrap();
        
        window = glfwCreateWindow(WIDTH as i32, HEIGHT as i32, title.as_ptr(), monitor, share);
        if window.is_null() {
            glfwTerminate();
            panic!("Failed to create GLFW window!");
        }
        
        glfwMakeContextCurrent(window);

        let gl: GL = GL::bind();

        glfwSwapInterval(1);
        Renderer { gl, window }
    }

    pub unsafe fn clear(&self) {
        gl!(self.gl.clear(GL_COLOR_BUFFER_BIT));
    }
    
    pub unsafe fn draw(&self, vertex_array: &VertexArray, index_buffer: &IndexBuffer, shader: &Shader) {
        shader.bind();
        index_buffer.bind();
        vertex_array.bind();    
        gl!(self.gl.draw_elements(GL_TRIANGLES, *index_buffer.get_index_count(), GL_UNSIGNED_INT, 0 as *mut c_void));
    }

    pub unsafe fn set_blend_func(&self) {
        gl!(self.gl.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA));
        gl!(self.gl.enable(GL_BLEND));
    }
}