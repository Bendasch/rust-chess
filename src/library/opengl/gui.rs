use crate::library::glfw::*;
use crate::library::opengl::{
    renderer::*,
    opengl::*,
    vertex_buffer::*,
    index_buffer::*,
    vertex_array::*,
    vertex_buffer_layout::*,
    shader::*,
    texture::*,
    gl_maths::*,
};
use std::{
    ffi::{CString},
    ptr::{null_mut},
    mem::size_of,
};
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
    
    let proj = ortho(0.0, WIDTH, 0.0, HEIGHT, -0.5, 0.5);
    let view = translate(0.0, 0.0, 0.0);
    let mut model: Mat4;
    
    while glfwWindowShouldClose(window) == 0 {
        
        renderer.clear();       
        
        shader.bind();

        {
            model = translate(WIDTH/2.0, HEIGHT/2.0, 0.0);
            let mvp = proj * view * model;
            shader.set_uniform_mat4f("u_MVP", mvp);
            renderer.draw(&vertex_array, &index_buffer, &shader);
        }
        
        {
            model = translate(-WIDTH/2.0, -HEIGHT/2.0, 0.0);
            let mvp = proj * view * model;        
            shader.set_uniform_mat4f("u_MVP", mvp);
            renderer.draw(&vertex_array, &index_buffer, &shader);
        }

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