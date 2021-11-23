use crate::gl;
use crate::library::gui::{opengl::*, utils::*, vertex_buffer::*};
use libc::{c_uint, c_void};
use std::rc::Rc;

pub struct VertexArray {
    gl: Rc<GL>,
    array_id: c_uint,
}

impl VertexArray {
    pub fn new(gl: Rc<GL>) -> VertexArray {
        let mut id: c_uint = 0;
        unsafe {
            gl!(gl.gen_vertex_arrays(1, &mut id));
        }
        VertexArray { gl, array_id: id }
    }

    pub fn add_buffer(&mut self, vertex_buffer: &VertexBuffer, layout: &VertexBufferLayout) {
        self.bind();
        vertex_buffer.bind();
        let elements = layout.elements();
        let mut offset: u32 = 0;
        for (i, element) in elements.iter().enumerate() {
            unsafe {
                gl!(self
                    .gl
                    .enable_vertex_array_attrib(*layout.vb_id(), i as u32));
                gl!(self.gl.vertex_attrib_pointer(
                    i as GLuint,
                    element.count,
                    element.gl_type,
                    element.normalized,
                    *layout.stride(),
                    offset as *mut c_void
                ));
            }
            offset += element.count as u32 * element.size as u32;
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl!(self.gl.bind_vertex_array(self.array_id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl!(self.gl.bind_vertex_array(0));
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_arrays(1, &self.array_id);
        }
    }
}
