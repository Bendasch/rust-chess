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

use crate::library::game::*;
use crate::library::gui::{
    glfw::*,
    renderer::{Renderer, callback},
    utils::print_opengl_version
};
use std::collections::LinkedList;

pub unsafe fn run(fen: Option<String>) {

    let mut game: LinkedList<State> = LinkedList::new();
    game.push_back(State::new(fen));

    let mut renderer: Renderer = Renderer::init(game);

    print_opengl_version(&renderer.gl);
    //print_opengl_extensions(&renderer.gl);
    
    glfwSetMouseButtonCallback(renderer.window, callback);

    while glfwWindowShouldClose(renderer.window) == 0 {
        
        renderer.clear();       
        
        // who to move?
        /* 
        match game.back().unwrap().check_game_over() {
            GameOver::BlackWon => {println!("Checkmate, black won!"); return},
            GameOver::WhiteWon => {println!("Checkmate, white won!"); return}, 
            GameOver::Stalemate => {println!("Stalemate!"); return},
            _ => draw_who_to_move(game.back().unwrap().turn())
        }
        */

        renderer.update();
        
        renderer.draw();

        glfwSwapBuffers(renderer.window);
        
        glfwPollEvents();
    }
    
    drop(renderer);
    glfwTerminate();
}