pub mod opengl;
pub mod renderer;
pub mod vertex_buffer;
pub mod index_buffer;
pub mod vertex_array;
pub mod vertex_buffer_layout;
pub mod shader;
pub mod utils;
pub mod texture;
pub mod gl_maths;


use crate::library::glfw::*;
use crate::library::gui::{
    renderer::*,
    vertex_buffer::*,
    index_buffer::*,
    vertex_array::*,
    vertex_buffer_layout::*,
    shader::*,
    texture::*,
    gl_maths::*,
};
use std::mem::size_of;
use libc::{c_uint, c_float, c_void};


pub unsafe fn run() {

    // initialize glfw context
    // create a window
    // load the opengl function pointers
    let renderer = Renderer::new();

    //print_opengl_version(&renderer.gl);
    //print_opengl_extensions(&renderer.gl);

    let positions: Vec<c_float> = Vec::from([
        0.0, 0.0,       0.0, 0.0,
        WIDTH, 0.0,     1.0, 0.0,
        WIDTH, HEIGHT,   1.0, 1.0,
        0.0, HEIGHT,     0.0, 1.0,
    ]);

    let vertex_buffer = VertexBuffer::new(positions.as_ptr() as *const c_void, (positions.len() * size_of::<c_float>()) as i32, &renderer.gl);
    
    let mut vertex_array = VertexArray::new(&renderer.gl);
    let mut layout = VertexBufferLayout::new();
    layout.push::<f32>(2);
    layout.push::<f32>(2);
    vertex_array.add_buffer(&vertex_buffer, &layout);

    let indices: Vec<c_uint> = Vec::from([
        0, 1, 2,
        2, 3, 0,
    ]);

    let index_buffer = IndexBuffer::new(indices.as_ptr() as *const c_void, indices.len() as i32, &renderer.gl);
    
    let mut shader = Shader::new(String::from("./src/library/opengl/simple.shader"), &renderer.gl);
    shader.bind();
    
    let texture = Texture::new("./res/partyinmytummy.png", &renderer.gl);
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
    
    while glfwWindowShouldClose(renderer.window) == 0 {
        
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

        glfwSwapBuffers(renderer.window);
        
        glfwPollEvents();
    }
    
    drop(shader);
    drop(vertex_buffer);
    drop(index_buffer);
    drop(vertex_array);
    drop(texture);
    
    glfwTerminate();
}