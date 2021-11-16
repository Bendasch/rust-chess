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
    mem::size_of,
    rc::Rc,
};
use libc::{c_void, c_float, c_uint};

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
    //texture: Texture
}

impl  Renderer {
    
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
        let positions: Vec<c_float> = Vec::from([
            /* 
            WIDTH/4.0, HEIGHT/4.0,   0.0, 0.0,
            WIDTH/2.0, HEIGHT/4.0,   1.0, 0.0,
            WIDTH/2.0, HEIGHT/2.0,   1.0, 1.0,
            WIDTH/4.0, HEIGHT/2.0,   0.0, 1.0,
            
            WIDTH/2.0,      HEIGHT/2.0,     0.0, 0.0,
            3.0*WIDTH/4.0,  HEIGHT/2.0,     1.0, 0.0, 
            3.0*WIDTH/4.0,  3.0*HEIGHT/4.0, 1.0, 1.0,
            WIDTH/2.0,      3.0*HEIGHT/4.0, 0.0, 1.0,
            */
            WIDTH/4.0,      HEIGHT/4.0,     1.0, 0.76, 0.53, 1.0,
            WIDTH/2.0,      HEIGHT/4.0,     1.0, 0.76, 0.53, 1.0,
            WIDTH/2.0,      HEIGHT/2.0,     1.0, 0.76, 0.53, 1.0,
            WIDTH/4.0,      HEIGHT/2.0,     1.0, 0.76, 0.53, 1.0,
            
            WIDTH/2.0,      HEIGHT/2.0,     0.5, 0.36, 0.73, 1.0,
            3.0*WIDTH/4.0,  HEIGHT/2.0,     0.5, 0.36, 0.73, 1.0,
            3.0*WIDTH/4.0,  3.0*HEIGHT/4.0, 0.5, 0.36, 0.73, 1.0,
            WIDTH/2.0,      3.0*HEIGHT/4.0, 0.5, 0.36, 0.73, 1.0,
        ]);
    
        let vertex_buffer = VertexBuffer::new(positions.as_ptr() as *const c_void, (positions.len() * size_of::<c_float>()) as i32, Rc::clone(&gl));
        
        let mut vertex_array = VertexArray::new(Rc::clone(&gl));
        let mut layout = VertexBufferLayout::new(vertex_buffer.buffer_id);
        layout.push::<f32>(2);
        layout.push::<f32>(4);
        vertex_array.add_buffer(&vertex_buffer, &layout);
    
        let indices: Vec<c_uint> = Vec::from([
            0, 1, 2,
            2, 3, 0,

            4, 5, 6,
            6, 7, 4,
        ]);
    
        let index_buffer = IndexBuffer::new(indices.as_ptr() as *const c_void, indices.len() as i32, Rc::clone(&gl));
        
        let shader = Shader::new(String::from("./src/library/gui/simple.shader"), Rc::clone(&gl));
        shader.bind();
        
        /*
        let texture = Texture::new("./src/library/gui/res/img/partyinmytummy.png", Rc::clone(&gl));
        texture.bind(0);
        Renderer::set_blend_func(Rc::clone(&gl));
        shader.set_uniform_1i("u_Texture", 0);
        */

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
            //exture
        }
    }

    pub unsafe fn clear(&self) {
        gl!(self.gl.clear(GL_COLOR_BUFFER_BIT));
    }
    
    pub unsafe fn update(&mut self) {    
        let mvp = ortho(0.0, WIDTH, 0.0, HEIGHT, -0.5, 0.5);
        self.shader.bind();
        self.shader.set_uniform_mat4f("u_MVP", mvp);
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
}