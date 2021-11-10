use crate::gl;
use crate::library::opengl::opengl::*;
use crate::library::opengl::renderer::*;
use libc::{c_void, c_uint};
use std::mem::size_of;

pub struct IndexBuffer<'a> {
    gl: &'a GL,
    buffer_id: c_uint,
    index_count: c_uint
}

impl<'a> IndexBuffer<'a> {

    pub unsafe fn new(data_ptr: *const c_void, index_count: c_uint, gl: &'a GL) -> IndexBuffer {
        
        let mut buffer_id: c_uint = 0;
        gl!(gl.gen_buffers(1, &mut buffer_id));
        gl!(gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, buffer_id));
        gl!(gl.buffer_data(GL_ELEMENT_ARRAY_BUFFER, (index_count * size_of::<c_uint>() as u32) as GLsizeiptr, data_ptr, GL_STATIC_DRAW));
        
        IndexBuffer { gl, buffer_id, index_count }
    }

    pub unsafe fn bind(&self) {
        gl!(self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.buffer_id));
    }

    pub unsafe fn unbind(&self) {
        gl!(self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0));
    }

    pub fn get_index_count(&self) -> &c_uint {
        &self.index_count
    }
}

impl<'a> Drop for IndexBuffer<'a> {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &mut self.buffer_id);
    }
}