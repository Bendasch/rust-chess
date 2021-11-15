use crate::gl;
use crate::library::gui::opengl::*;
use crate::library::gui::utils::*;
use libc::{c_void, c_uint};

pub struct VertexBuffer<'a> {
    gl: &'a GL,
    buffer_id: c_uint,
}

impl<'a> VertexBuffer<'a> {

    pub unsafe fn new(data_ptr: *const c_void, data_size: i32, gl: &'a GL) -> VertexBuffer {
        
        let mut buffer_id: c_uint = 0;
        gl!(gl.gen_buffers(1, &mut buffer_id));
        gl!(gl.bind_buffer(GL_ARRAY_BUFFER, buffer_id));
        gl!(gl.buffer_data(GL_ARRAY_BUFFER, data_size as GLsizeiptr, data_ptr, GL_STATIC_DRAW));
        
        VertexBuffer { gl, buffer_id }
    }

    pub unsafe fn bind(&self) {
        gl!(self.gl.bind_buffer(GL_ARRAY_BUFFER, self.buffer_id));
    }

    pub unsafe fn unbind(&self) {
        gl!(self.gl.bind_buffer(GL_ARRAY_BUFFER, 0));
    }
}

impl<'a> Drop for VertexBuffer<'a> {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &mut self.buffer_id);
    }
}