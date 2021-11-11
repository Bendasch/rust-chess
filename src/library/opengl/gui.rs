use crate::library::opengl::renderer::*;
use crate::library::glfw::*;
use crate::library::opengl::opengl::*;
use crate::library::opengl::vertex_buffer::*;
use crate::library::opengl::index_buffer::*;
use crate::library::opengl::vertex_array::*;
use crate::library::opengl::vertex_buffer_layout::*;
use crate::library::opengl::shader::*;
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

    let positions: [c_float; 8] = [
        -0.5,  -0.5, 
        0.5,  -0.5, 
        0.5,   0.5,
        -0.5,   0.5,
    ];

    let vertex_buffer = VertexBuffer::new(positions.as_ptr() as *const c_void, (8 * size_of::<c_float>()) as i32, &gl);
    
    let mut vertex_array = VertexArray::new(&gl);
    let mut layout = VertexBufferLayout::new();
    layout.push::<f32>(2);
    vertex_array.add_buffer(&vertex_buffer, &layout);

    let indices: [c_uint; 6] = [
        0, 1, 2,
        2, 3, 0
    ];
    
    let index_buffer = IndexBuffer::new(indices.as_ptr() as *const c_void, 6, &gl);
    
    let mut shader = Shader::new(String::from("./src/library/opengl/simple.shader"), &gl);
    
    vertex_array.unbind();
    vertex_buffer.unbind();    
    index_buffer.unbind();
    shader.unbind();

    let mut red = 0.5f32;
    let mut red_increment = 0.005f32;
    let mut green = 0.25f32;
    let mut green_increment = 0.001f32;
    let mut blue = 0.65f32;
    let mut blue_increment = 0.01f32;

    while glfwWindowShouldClose(window) == 0 {
        
        if red > 0.9 || red < 0.1 {
            red_increment *= -1.0f32; 
        }
        red += red_increment;

        if green > 0.7 || green < 0.3 {
            green_increment *= -1.0f32;
        }
        green += green_increment;

        if blue > 0.95 || blue < 0.05 {
            blue_increment *= -1.0f32;
        }
        blue += blue_increment;

        renderer.clear();          
        shader.bind();
        shader.set_uniform_4f("u_Color", red, green, blue, 0.9f32);
        renderer.draw(&vertex_array, &index_buffer, &shader);

        glfwSwapBuffers(window);
        
        glfwPollEvents();
    }
    
    drop(shader);
    drop(vertex_buffer);
    drop(index_buffer);
    drop(vertex_array);
    
    glfwTerminate();
}