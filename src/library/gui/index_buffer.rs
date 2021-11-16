use crate::gl;
use crate::library::gui::opengl::*;
use crate::library::gui::utils::*;
use libc::{c_void, c_uint};
use std::{
    mem::size_of,
    rc::Rc,
};

pub struct IndexBuffer {
    gl: Rc<GL>,
    buffer_id: c_uint,
    index_count: i32
}

impl IndexBuffer {

    pub unsafe fn new(data_ptr: *const c_void, index_count: i32, gl: Rc<GL>) -> IndexBuffer {
        
        let mut buffer_id: c_uint = 0;
        gl!(gl.gen_buffers(1, &mut buffer_id));
        gl!(gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, buffer_id));
        gl!(gl.buffer_data(GL_ELEMENT_ARRAY_BUFFER, (index_count * size_of::<c_uint>() as i32) as GLsizeiptr, data_ptr, GL_STATIC_DRAW));
        
        IndexBuffer { gl, buffer_id, index_count }
    }

    pub unsafe fn bind(&self) {
        gl!(self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.buffer_id));
    }

    pub unsafe fn unbind(&self) {
        gl!(self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0));
    }

    pub fn get_index_count(&self) -> &i32 {
        &self.index_count
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &mut self.buffer_id);
    }
}