use crate::library::opengl::opengl::*;
use std::mem::size_of;
use std::any::TypeId;

#[derive(Debug)]
pub struct VertexBufferElement {
    pub gl_type: GLenum,
    pub count: GLint,
    pub normalized: GLboolean,
    pub size: GLsizei
}

pub struct VertexBufferLayout {
    elements: Vec<VertexBufferElement>,
    stride: GLsizei,
}


impl<'b> VertexBufferLayout {

    pub fn new() -> VertexBufferLayout { 
        VertexBufferLayout {elements: Vec::new(), stride: 0}
    }

    pub fn elements(&self) -> &Vec<VertexBufferElement> {
        &self.elements
    }
    
    pub fn stride(&self) -> &GLsizei {
        &self.stride
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
