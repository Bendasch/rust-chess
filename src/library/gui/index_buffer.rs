use crate::gl;
use crate::library::gui::{opengl::*, utils::*};
use libc::{c_uint, c_void};
use std::{mem::size_of, ptr::null, rc::Rc};

pub struct IndexBuffer {
    gl: Rc<GL>,
    buffer_id: c_uint,
    index_count: i32,
}

impl IndexBuffer {
    pub fn new_static(data: &[u32], index_count: i32, gl: Rc<GL>) -> IndexBuffer {
        unsafe {
            let mut buffer_id: c_uint = 0;

            gl!(gl.gen_buffers(1, &mut buffer_id));
            gl!(gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, buffer_id));
            gl!(gl.buffer_data(
                GL_ELEMENT_ARRAY_BUFFER,
                (index_count * size_of::<c_uint>() as i32) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                GL_STATIC_DRAW
            ));

            IndexBuffer {
                gl,
                buffer_id,
                index_count,
            }
        }
    }

    pub fn new(index_count: i32, gl: Rc<GL>) -> IndexBuffer {
        unsafe {
            let mut buffer_id: c_uint = 0;
            gl!(gl.gen_buffers(1, &mut buffer_id));
            gl!(gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, buffer_id));
            gl!(gl.buffer_data(
                GL_ELEMENT_ARRAY_BUFFER,
                (index_count * size_of::<c_uint>() as i32) as GLsizeiptr,
                null::<c_void>(),
                GL_DYNAMIC_DRAW
            ));

            IndexBuffer {
                gl,
                buffer_id,
                index_count,
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl!(self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.buffer_id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl!(self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0));
        }
    }

    pub fn get_index_count(&self) -> &i32 {
        &self.index_count
    }

    pub fn buffer_sub_data(&self, data: &[u32], data_amount: usize, offset: usize) {
        unsafe {
            gl!(self.gl.buffer_sub_data(
                GL_ELEMENT_ARRAY_BUFFER,
                (size_of::<c_uint>() * offset) as i32,
                (size_of::<c_uint>() * data_amount) as i32,
                data.as_ptr() as *const c_void
            ));
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffers(1, &self.buffer_id);
        }
    }
}
