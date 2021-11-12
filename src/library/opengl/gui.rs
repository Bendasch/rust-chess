use crate::library::opengl::renderer::*;
use crate::library::glfw::*;
use crate::library::opengl::opengl::*;
use crate::library::opengl::vertex_buffer::*;
use crate::library::opengl::index_buffer::*;
use crate::library::opengl::vertex_array::*;
use crate::library::opengl::vertex_buffer_layout::*;
use crate::library::opengl::shader::*;
use crate::library::opengl::texture::*;
use std::ffi::{CString};
use std::ptr::{null_mut};
use std::mem::size_of;
use libc::{c_uint, c_float, c_void};

pub unsafe fn run() {
    
    let window: *mut GLFWwindow;
    let monitor: *mut GLFWmonitor = null_mut();
    let share: *mut GLFWwindow = null_mut();
    
    if glfwInit() == 0 {
        return;
    }
    
    let title = CString::new("Rust chess (OpenGL)").unwrap();
    
    window = glfwCreateWindow(640, 480, title.as_ptr(), monitor, share);
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
        -0.5, -0.5, 0.0, 0.0,
        0.5, -0.5, 1.0, 0.0,
        0.45, 0.45, 0.95, 0.95,
        -0.5, 0.5, 0.0, 1.0,
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
    
    let mut shader = Shader::new(String::from("./src/library/opengl/simple.shader"), &gl);
    shader.bind();
    
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