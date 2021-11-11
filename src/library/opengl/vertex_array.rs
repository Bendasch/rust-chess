use crate::gl;
use crate::library::opengl::utils::*;
use crate::library::opengl::opengl::*;
use crate::library::opengl::vertex_buffer::*;
use crate::library::opengl::vertex_buffer_layout::*;
use libc::{c_uint, c_void};

pub struct VertexArray<'a> {
    gl: &'a GL,
    array_id: c_uint, 
}

impl<'a, 'b> VertexArray<'a> {

    pub unsafe fn new(gl: &'a GL) -> VertexArray<'a> {
        let mut id: c_uint = 0;
        gl!(gl.gen_vertex_arrays(1, &mut id));
        VertexArray { gl, array_id: id }
    }

    pub unsafe fn add_buffer(&mut self, vertex_buffer: &'b VertexBuffer, layout: &'b VertexBufferLayout) {
        self.bind();
        vertex_buffer.bind();    
        let elements = layout.elements();
        let mut offset: u32 = 0;
        for (i, element) in elements.iter().enumerate() {
            gl!(self.gl.enable_vertex_attrib_array(i as u32));
            gl!(self.gl.vertex_attrib_pointer(i as GLuint, element.count, element.gl_type, element.normalized, *layout.stride(), offset as *mut c_void));
            offset += element.count as u32 * element.size as u32;
        }
    }

    pub unsafe fn bind(&self) {
        gl!(self.gl.bind_vertex_array(self.array_id));
    }
    
    pub unsafe fn unbind(&self) {
        gl!(self.gl.bind_vertex_array(0));
    }
}

impl<'a> Drop for VertexArray<'a> {
    fn drop(&mut self) {
        self.gl.delete_vertex_arrays(1, &self.array_id);
    }
}