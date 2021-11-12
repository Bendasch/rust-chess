use crate::gl;
use crate::library::opengl::opengl::*;
use crate::library::opengl::utils::*;
use crate::library::opengl::vertex_array::*;
use crate::library::opengl::index_buffer::*;
use crate::library::opengl::shader::*;
use libc::{c_void};

pub struct Renderer<'a> {
    gl: &'a GL
}

impl<'a> Renderer<'a> {
    
    pub fn new(gl: &'a GL) -> Renderer {
        Renderer { gl }
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