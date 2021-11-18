use crate::gl;
use crate::library::gui::{
    glfw::*,
    opengl::*,
    index_buffer::*,
    vertex_buffer::*,
    vertex_buffer_layout::*,
    vertex_array::*,
    shader::*,
    utils::*,
    gl_maths::*,
    texture::*
};
use std::{
    ffi::{CString},
    ptr::{null_mut},
    rc::Rc,
};
use libc::{c_void, c_uint};

pub static WIDTH: f32 = 1024.0;
pub static HEIGHT: f32 = 768.0;

#[allow(dead_code)]
pub struct Renderer {
    pub gl: Rc<GL>,
    pub window: *mut GLFWwindow,
    shader: Shader,
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    textures: Vec<Texture>
}

impl Renderer {
    
    pub unsafe fn init() -> Renderer  {
        
        let window: *mut GLFWwindow;
        let monitor: *mut GLFWmonitor = null_mut();
        let share: *mut GLFWwindow = null_mut();
        
        if glfwInit() == 0 {
            panic!("Failed to initialize GLFW!");
        }
        
        let title = CString::new("Rust chess (OpenGL)").unwrap();
        
        window = glfwCreateWindow(WIDTH as i32, HEIGHT as i32, title.as_ptr(), monitor, share);
        if window.is_null() {
            glfwTerminate();
            panic!("Failed to create GLFW window!");
        }
        
        glfwMakeContextCurrent(window);

        let gl = Rc::new(GL::bind());

        glfwSwapInterval(1);
        
        let vertex_buffer = VertexBuffer::new(null_mut() as *const c_void, 1000, Rc::clone(&gl));
        let mut layout = VertexBufferLayout::new(vertex_buffer.buffer_id);
        layout.push::<f32>(3);
        layout.push::<f32>(4);
        layout.push::<f32>(2);
        layout.push::<f32>(1);
        
        let mut vertex_array = VertexArray::new(Rc::clone(&gl));
        vertex_array.add_buffer(&vertex_buffer, &layout);
    
        let indices: Vec<c_uint> = Vec::from([
            0, 1, 2,
            2, 3, 0,

            4, 5, 6,
            6, 7, 4,
        ]);
    
        let index_buffer = IndexBuffer::new(indices.as_ptr() as *const c_void, indices.len() as i32, Rc::clone(&gl));
        
        let mut shader = Shader::new(String::from("./src/library/gui/simple.shader"), Rc::clone(&gl));
        shader.bind();
    
        let tummy = Texture::new("./src/library/gui/res/img/partyinmytummy.png", Rc::clone(&gl));
        let texture = Texture::new("./src/library/gui/res/img/texture.png", Rc::clone(&gl));
        shader.set_uniform_1iv("u_Textures", vec![0, 1]);
        
        Renderer::set_blend_func(Rc::clone(&gl));

        vertex_array.unbind();
        vertex_buffer.unbind();    
        index_buffer.unbind();
        shader.unbind();

        Renderer { 
            gl, 
            window,
            vertex_array,
            vertex_buffer,
            index_buffer,
            shader, 
            textures: vec![tummy, texture]
        }
    }

    pub unsafe fn clear(&self) {
        gl!(self.gl.clear(GL_COLOR_BUFFER_BIT));
    }
    
    pub unsafe fn update(&mut self) {    

        let mvp = ortho(0.0, WIDTH, 0.0, HEIGHT, -0.5, 0.5);
        self.shader.bind();
        self.shader.set_uniform_mat4f("u_MVP", mvp);
        self.textures[0].bind_texture_unit(0);
        self.textures[1].bind_texture_unit(1);
        //self.textures[0].bind(0);
        //self.textures[1].bind(1);
        
        /*
        let positions: Vec<c_float> = Vec::from([
            WIDTH/4.0,      HEIGHT/4.0,         0.0,    1.0, 0.76, 0.53, 1.0,   0.0, 0.0,   0.0,
            WIDTH/2.0,      HEIGHT/4.0,         0.0,    1.0, 0.76, 0.53, 1.0,   1.0, 0.0,   0.0,
            WIDTH/2.0,      HEIGHT/2.0,         0.0,    1.0, 0.76, 0.53, 1.0,   1.0, 1.0,   0.0,
            WIDTH/4.0,      HEIGHT/2.0,         0.0,    1.0, 0.76, 0.53, 1.0,   0.0, 1.0,   0.0,
            WIDTH/2.0,      HEIGHT/2.0,         0.0,    0.5, 0.36, 0.73, 1.0,   0.0, 0.0,   1.0,
            3.0*WIDTH/4.0,  HEIGHT/2.0,         0.0,    0.5, 0.36, 0.73, 1.0,   1.0, 0.0,   1.0,
            3.0*WIDTH/4.0,  3.0*HEIGHT/4.0,     0.0,    0.5, 0.36, 0.73, 1.0,   1.0, 1.0,   1.0,
            WIDTH/2.0,      3.0*HEIGHT/4.0,     0.0,    0.5, 0.36, 0.73, 1.0,   0.0, 1.0,   1.0,
            ]);
        */
        
        let positions = Renderer::get_board_vertices();

        self.vertex_buffer.bind();
        self.vertex_buffer.buffer_sub_data(positions.as_ptr() as *const c_void, positions.len());
    }

    pub unsafe fn draw(&self) {
        self.shader.bind();
        self.index_buffer.bind();
        self.vertex_array.bind();    
        gl!(self.gl.draw_elements(GL_TRIANGLES, *self.index_buffer.get_index_count(), GL_UNSIGNED_INT, 0 as *mut c_void));
    }

    pub unsafe fn set_blend_func(gl: Rc<GL>) {
        gl!(gl.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA));
        gl!(gl.enable(GL_BLEND));
    }

    pub fn get_board_vertices() -> Vec<f32> {
        let mut vertices = Vec::new();
        for i in 0..=8 {
            for j in 0..=8 {

                let mut texture_id: f32;

                // lower left square
                if i > 0 && j > 0 {
                    texture_id = (i + j + 1 % 2) as f32;
                    vertices.push(Vertex {
                        position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0, 0.0),
                        color: RGBA(1.0, 1.0, 1.0, 1.0),
                        texture_coords: TextureCoords(1.0, 1.0),
                        texture_id,
                    });
                }

                // lower right square
                if i < 8 && j > 0{
                    texture_id = (i + j % 2) as f32;
                    vertices.push(Vertex {
                        position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0, 0.0),
                        color: RGBA(1.0, 1.0, 1.0, 1.0),
                        texture_coords: TextureCoords(0.0, 1.0),
                        texture_id,
                    });
                }
                
                // upper right square
                if i < 8 && j < 8 {
                    texture_id = (i + j + 1 % 2) as f32;
                    vertices.push(Vertex {
                        position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0, 0.0),
                        color: RGBA(1.0, 1.0, 1.0, 1.0),
                        texture_coords: TextureCoords(0.0, 0.0),
                        texture_id,
                    });
                }

                // upper left square
                if i > 0 && j < 8 {
                    texture_id = (i + j % 2) as f32;
                    vertices.push(Vertex {
                        position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0, 0.0),
                        color: RGBA(1.0, 1.0, 1.0, 1.0),
                        texture_coords: TextureCoords(1.0, 0.0),
                        texture_id,
                    });
                }
            }
        }

        Renderer::deserialize(vertices)
    }

    pub fn get_board_indices() -> Vec<u32> {
        let mut indices = Vec::new();
        for i in 0..8 {
            for j in 0..8 {

                /* 
                    We need to account for the fact that multiple vertices were created
                    in order to have hard texture edges between squares. There are 1, 2 or 4 
                    copies per vertex position.
                    
                    The number of copies per vertex position are laid out like this:
                    1 2 2 2 2 2 2 2 1
                    2 4 4 4 4 4 4 4 2
                    2 4 4 4 4 4 4 4 2
                    2 4 4 4 4 4 4 4 2
                    2 4 4 4 4 4 4 4 2
                    2 4 4 4 4 4 4 4 2
                    2 4 4 4 4 4 4 4 2
                    2 4 4 4 4 4 4 4 2
                    1 2 2 2 2 2 2 2 1

                    However, the number of copies per vertex position does not match the number of triangles
                    that are connected to this vertex position. 1, 2, 3 or 6 triangles have to be drawn for each
                    vertex position. 

                    Vertex positions with 1 copy are part of either 1 or 2 triangles:
                        > Upper-left and lower-right corner are part of 1 triangle.
                        > Upper-right and lower-left corner are part of 2 triangles.
                    Vertex positions with 2 copies are part of 3 triangles.
                    Vertex positions with 4 copies are part of 6 triangles.

                    This is due to the counter-clockwise composition of the triangles.

                    Now we need to find the right indices to make this happen...
                     
                    This is the initial solution for when only one copy of a vertex position is used.
                    indices.push(i      + 9 * j);
                    indices.push(i + 1  + 9 * j);
                    indices.push(i + 1  + 9 * (j + 1));
                    
                    indices.push(i + 1  + 9 * (j + 1));
                    indices.push(i      + 9 * (j + 1));
                    indices.push(i      + 9 * j);

                    There are now 16 verteces on the edge rows and columns and 32 verteces in the inside rows
                    and columns. There are a total of 256 verteces now (rather than 81).
                */ 
                
                let rank_offset = match i {
                    0 => 0,
                    7 => (256-32) as u32,
                    _ => 16 + (32 * (i-1)) as u32,
                };
                
                // etc etc...
            }
        }
        indices
    }

    pub fn get_quad_vertices(x: f32, y: f32, w: f32, h: f32) -> Vec<Vertex> {
    
        let v1 = Vertex {
            position: Position(x, y, 0.0),
            color: RGBA(1.0, 1.0, 1.0, 1.0),
            texture_coords: TextureCoords(0.0, 0.0),
            texture_id: 0.0,
        };
    
        let v2 = Vertex {
            position: Position(x + w, y, 0.0),
            color: RGBA(1.0, 1.0, 1.0, 1.0),
            texture_coords: TextureCoords(1.0, 0.0),
            texture_id: 0.0,
        };
    
        let v3 = Vertex {
            position: Position(x + w, y + h, 0.0),
            color: RGBA(1.0, 1.0, 1.0, 1.0),
            texture_coords: TextureCoords(1.0, 1.0),
            texture_id: 0.0,
        };
    
        let v4 = Vertex {
            position: Position(x, y + h, 0.0),
            color: RGBA(1.0, 1.0, 1.0, 1.0),
            texture_coords: TextureCoords(0.0, 1.0),
            texture_id: 0.0,
        };
    
        vec![v1, v2, v3, v4]
    }
    
    pub fn deserialize(vertices: Vec<Vertex>) -> Vec<f32> {
        let mut result = Vec::new();
        for mut vertex in vertices {
            result.append(&mut vertex.deserialize());
        }
        result
    }
}