extern crate glm;
use crate::gl;
use crate::library::gui::{
    opengl::*,
    utils::*,
};
use libc::{c_void, c_uint};
use std::{rc::Rc, mem::size_of, any::TypeId};

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

    pub unsafe fn buffer_sub_data(&self, data_ptr: *const c_void, data_amount: usize, offset: usize) {
        gl!(self.gl.buffer_sub_data(GL_ARRAY_BUFFER, (size_of::<Vertex>() * offset) as i32, (size_of::<f32>() * data_amount) as i32, data_ptr));
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &self.buffer_id);
    }
}

#[derive(Debug)]
pub struct VertexBufferElement {
    pub gl_type: GLenum,
    pub count: GLint,
    pub normalized: GLboolean,
    pub size: GLsizei,
}

pub struct VertexBufferLayout {
    vertex_buffer_id: GLuint,
    elements: Vec<VertexBufferElement>,
    stride: GLsizei,
}


impl<'b> VertexBufferLayout {

    pub fn new(vertex_buffer_id: GLuint) -> VertexBufferLayout { 
        VertexBufferLayout {vertex_buffer_id, elements: Vec::new(), stride: 0}
    }

    pub fn elements(&self) -> &Vec<VertexBufferElement> {
        &self.elements
    }
    
    pub fn stride(&self) -> &GLsizei {
        &self.stride
    } 

    pub fn vb_id(&self) -> &GLuint {
        &self.vertex_buffer_id
    }
    
    pub fn push<T: 'static>(&mut self, count: GLint) {

        let size = size_of::<T>() as GLsizei;
        
        let gl_type: GLenum;
        let typ = TypeId::of::<T>();
        if typ == TypeId::of::<f32>() {
            gl_type = GL_FLOAT
        } else if typ == TypeId::of::<u32>() {
            gl_type = GL_UNSIGNED_INT
        } else {
            panic!("Not supported yet...")
        }

        let element = VertexBufferElement { gl_type, count, normalized: GL_FALSE, size };

        self.elements.push(element);
        self.stride += count * size;
    }
}

pub struct Position(pub f32, pub f32);
//pub struct RGBA(pub f32, pub f32, pub f32, pub f32);
pub struct TextureCoords(pub f32, pub f32);

pub struct Vertex {
    pub position: Position,
    pub texture_coords: TextureCoords,
    pub texture_id: f32,
}

impl Vertex {
    pub fn deserialize(&mut self) -> Vec<f32> {
        vec![
            self.position.0,
            self.position.1,
            self.texture_coords.0,
            self.texture_coords.1,
            self.texture_id
        ]
    }
}

#[cfg(debug_assertions)]
pub fn print_vertices(vertices: &[f32]) {
    println!("------ NEW FRAME ------");
    for i in 0..vertices.len() / 5 {
        print!("x: {:?}", vertices[i*5]);
        print!(", y: {:?}", vertices[i*5+1]);
        print!(", u: {:?}", vertices[i*5+2]);
        print!(", v: {:?}", vertices[i*5+3]);
        println!(", texture: {:?}", vertices[i*5+4]);
    }
}