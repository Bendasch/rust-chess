pub mod glfw;
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

use crate::library::gui::{
    glfw::*,
    renderer::Renderer,
    utils::print_opengl_version
};

pub unsafe fn run() {

    let mut renderer = Renderer::init();

    print_opengl_version(&renderer.gl);
    //print_opengl_extensions(&renderer.gl);

    while glfwWindowShouldClose(renderer.window) == 0 {
        
        renderer.clear();       
        
        renderer.update();
        
        renderer.draw();
        
        glfwSwapBuffers(renderer.window);
        
        glfwPollEvents();
    }
    
    drop(renderer);
    glfwTerminate();
}