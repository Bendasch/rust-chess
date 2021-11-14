use crate::library::opengl::renderer::*;
use crate::library::glfw::*;
use crate::library::opengl::opengl::*;
use crate::library::opengl::vertex_buffer::*;
use crate::library::opengl::index_buffer::*;
use crate::library::opengl::vertex_array::*;
use crate::library::opengl::vertex_buffer_layout::*;
use crate::library::opengl::shader::*;
use crate::library::opengl::texture::*;
use crate::library::opengl::gl_maths::*;
use std::ffi::{CString};
use std::ptr::{null_mut};
use std::mem::size_of;
use libc::{c_uint, c_float, c_void};

static WIDTH: f32 = 1024.0;
static HEIGHT: f32 = 768.0;

pub unsafe fn run() {

    let window: *mut GLFWwindow;
    let monitor: *mut GLFWmonitor = null_mut();
    let share: *mut GLFWwindow = null_mut();
    
    if glfwInit() == 0 {
        return;
    }
    
    let title = CString::new("Rust chess (OpenGL)").unwrap();
    
    window = glfwCreateWindow(WIDTH as i32, HEIGHT as i32, title.as_ptr(), monitor, share);
    if window.is_null() {
        glfwTerminate();
        return;
    }
    
    glfwMakeContextCurrent(window);

    let gl: GL = GL::bind();
    let renderer = Renderer::new(&gl);

    glfwSwapInterval(1);
    
    //print_opengl_version(&gl);
    //print_opengl_extensions(&gl);

    let positions: Vec<c_float> = Vec::from([
        0.0, 0.0,       0.0, 0.0,
        WIDTH, 0.0,     1.0, 0.0,
        WIDTH, HEIGHT,   1.0, 1.0,
        0.0, HEIGHT,     0.0, 1.0,
    ]);

    let vertex_buffer = VertexBuffer::new(positions.as_ptr() as *const c_void, (positions.len() * size_of::<c_float>()) as i32, &gl);
    
    let mut vertex_array = VertexArray::new(&gl);
    let mut layout = VertexBufferLayout::new();
    layout.push::<f32>(2);
    layout.push::<f32>(2);
    vertex_array.add_buffer(&vertex_buffer, &layout);

    let indices: Vec<c_uint> = Vec::from([
        0, 1, 2,
        2, 3, 0,
    ]);

    let index_buffer = IndexBuffer::new(indices.as_ptr() as *const c_void, indices.len() as i32, &gl);
    
    let proj = ortho(0.0, WIDTH, 0.0, HEIGHT, -0.5, 0.5);
    let view = translate(0.0, 0.0, 0.0) * rotate_z(1.0);
    let mvp = proj * view;

    let mut shader = Shader::new(String::from("./src/library/opengl/simple.shader"), &gl);
    shader.bind();
    shader.set_uniform_mat4f("u_MVP", mvp);
    
    let texture = Texture::new("./res/partyinmytummy.png", &gl);
    texture.bind(0);
    renderer.set_blend_func();

    shader.set_uniform_1i("u_Texture", 0);

    vertex_array.unbind();
    vertex_buffer.unbind();    
    index_buffer.unbind();
    shader.unbind();

    while glfwWindowShouldClose(window) == 0 {

        renderer.clear();       
        
        renderer.draw(&vertex_array, &index_buffer, &shader);

        glfwSwapBuffers(window);
        
        glfwPollEvents();
    }
    
    drop(shader);
    drop(vertex_buffer);
    drop(index_buffer);
    drop(vertex_array);
    drop(texture);
    
    glfwTerminate();
}