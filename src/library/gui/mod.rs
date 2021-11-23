pub mod glfw;
pub mod index_buffer;
pub mod maths;
pub mod opengl;
pub mod renderer;
pub mod shader;
pub mod texture;
pub mod utils;
pub mod vertex_array;
pub mod vertex_buffer;

use crate::library::{
    game::*,
    gui::{glfw::*, renderer::*, utils::print_opengl_version},
};
use std::collections::LinkedList;

pub unsafe fn run(fen: Option<String>) {
    let mut game: LinkedList<State> = LinkedList::new();
    game.push_back(State::new(fen));

    let mut renderer: Renderer = Renderer::init(game);

    print_opengl_version(&renderer.gl);
    //print_opengl_extensions(&renderer.gl);

    glfwSetMouseButtonCallback(renderer.get_window(), click_callback);
    glfwSetFramebufferSizeCallback(renderer.get_window(), framebuffer_size_callback);

    while glfwWindowShouldClose(renderer.get_window()) == 0 {
        renderer.clear();
        renderer.draw();
        glfwSwapBuffers(renderer.get_window());
        glfwPollEvents();
    }

    drop(renderer);
    glfwTerminate();
}
