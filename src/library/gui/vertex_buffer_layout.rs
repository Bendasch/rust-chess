extern crate glm;
use crate::library::gui::opengl::*;
use std::mem::size_of;
use std::any::TypeId;

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
        let mut vec = Vec::new();
        vec.push(self.position.0);
        vec.push(self.position.1);
        vec.push(self.texture_coords.0);
        vec.push(self.texture_coords.1);
        vec.push(self.texture_id);
        vec
        /*
        vec.push(self.position.2);
        vec.push(self.color.0);
        vec.push(self.color.1);
        vec.push(self.color.2);
        vec.push(self.color.3);
        */
    }
}

#[cfg(debug_assertions)]
pub fn print_vertices(vertices: &Vec<f32>) {
    println!("------ NEW FRAME ------");
    for i in 0..vertices.len() / 5 {
        print!("x: {:?}", vertices[i*5]);
        print!(", y: {:?}", vertices[i*5+1]);
        print!(", u: {:?}", vertices[i*5+2]);
        print!(", v: {:?}", vertices[i*5+3]);
        println!(", texture: {:?}", vertices[i*5+4]);
    }
}