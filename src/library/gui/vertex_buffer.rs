use crate::gl;
use crate::library::gui::{
    opengl::*,
    utils::*,
    vertex_buffer_layout::*,
};
use libc::{c_void, c_uint};
use std::{rc::Rc, mem::size_of};

pub struct VertexBuffer {
    gl: Rc<GL>,
    pub buffer_id: c_uint,
}

impl VertexBuffer {

    pub unsafe fn new(data_ptr: *const c_void, vertex_amount: i32, gl: Rc<GL>) -> VertexBuffer {
        
        let mut buffer_id: c_uint = 0;
        gl!(gl.gen_buffers(1, &mut buffer_id));
        gl!(gl.bind_buffer(GL_ARRAY_BUFFER, buffer_id));

        let draw_type = if data_ptr.is_null() { GL_DYNAMIC_DRAW } else { GL_STATIC_DRAW };
          
        gl!(gl.buffer_data(GL_ARRAY_BUFFER, vertex_amount * size_of::<Vertex>() as i32, data_ptr, draw_type));
        
        VertexBuffer { gl, buffer_id }
    }

    pub unsafe fn bind(&self) {
        gl!(self.gl.bind_buffer(GL_ARRAY_BUFFER, self.buffer_id));
    }

    pub unsafe fn unbind(&self) {
        gl!(self.gl.bind_buffer(GL_ARRAY_BUFFER, 0));
    }

    pub unsafe fn buffer_sub_data(&self, data_ptr: *const c_void, data_amount: usize) {
        gl!(self.gl.buffer_sub_data(GL_ARRAY_BUFFER, 0, (size_of::<f32>() * data_amount) as i32, data_ptr));
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &mut self.buffer_id);
    }
}