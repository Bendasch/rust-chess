use crate::gl;
use crate::library::game::*;
use crate::library::gui::{
    glfw::*, index_buffer::*, maths::*, opengl::*, shader::*, texture::*, utils::*,
    vertex_array::*, vertex_buffer::*,
};
use libc::{c_int, c_void};
use std::{
    collections::LinkedList,
    ffi::CString,
    mem::forget,
    ptr::null_mut,
    rc::Rc,
    sync::{Arc, RwLock},
};

/*
    Currently, loads of things need to be refactored:
        - which methods need to be methods of the 'Renderer' struct?
        - how should state be passed to glfw?
        - how can the drawing be meaningfully abstracted?
*/

// TO DO: Refactor the way the renderer and game state are passed to glfw callbacks.
pub static mut WIDTH: f32 = 1024.0;
pub static mut HEIGHT: f32 = 768.0;

#[allow(dead_code)]
pub struct Renderer {
    pub gl: Rc<GL>,
    pub window: *mut GLFWwindow,
    shader: Shader,
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    textures: Vec<Texture>,
    game_state: Arc<RwLock<GameState>>,
}

pub struct GameState {
    pub selected_field: Option<(usize, usize)>,
    pub game: LinkedList<State>,
}

impl Renderer {
    // TO DO: Refactor this method...
    pub unsafe fn init(game: LinkedList<State>) -> Renderer {
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

        let vertex_buffer = VertexBuffer::new(452, Rc::clone(&gl));
        let mut layout = VertexBufferLayout::new(vertex_buffer.buffer_id);
        layout.push::<f32>(2);
        layout.push::<f32>(2);
        layout.push::<f32>(1);

        let mut vertex_array = VertexArray::new(Rc::clone(&gl));
        vertex_array.add_buffer(&vertex_buffer, &layout);

        let vertices = Renderer::get_board_vertices(&None);
        vertex_buffer.bind();
        vertex_buffer.buffer_sub_data(&vertices, vertices.len(), 0);

        let board_indices = Renderer::get_board_indices();
        let index_buffer = IndexBuffer::new(576, Rc::clone(&gl));
        index_buffer.bind();
        index_buffer.buffer_sub_data(&board_indices, board_indices.len(), 0);

        let mut shader = Shader::new(
            String::from("./src/library/gui/res/simple.shader"),
            Rc::clone(&gl),
        );
        shader.bind();

        let white_field = Texture::new("./src/library/gui/res/img/white_field.png", Rc::clone(&gl));
        let black_field = Texture::new("./src/library/gui/res/img/black_field.png", Rc::clone(&gl));
        let pieces = Texture::new("./src/library/gui/res/img/pieces.png", Rc::clone(&gl));
        shader.set_uniform_1iv("u_Textures", vec![0, 1, 2]);

        Renderer::set_blend_func(Rc::clone(&gl));

        vertex_array.unbind();
        vertex_buffer.unbind();
        index_buffer.unbind();
        shader.unbind();

        // "bind" the game state to the glfw window
        let game_state = Arc::new(RwLock::new(GameState {
            selected_field: None,
            game,
        }));

        glfwSetWindowUserPointer(window, Arc::as_ptr(&game_state) as *const c_void);

        Renderer {
            gl,
            window,
            vertex_array,
            vertex_buffer,
            index_buffer,
            shader,
            textures: vec![white_field, black_field, pieces],
            game_state,
        }
    }

    pub unsafe fn clear(&self) {
        gl!(self.gl.clear(GL_COLOR_BUFFER_BIT));
    }

    // TO DO: Refactor this method...
    pub unsafe fn update(&mut self) {
        let mvp = ortho(0.0, WIDTH, 0.0, HEIGHT, -0.5, 0.5);
        self.shader.bind();
        self.shader.set_uniform_mat4f("u_MVP", mvp);
        gl!(self.gl.viewport(0, 0, WIDTH as i32, HEIGHT as i32));
    }

    // TO DO: Refactor this method...
    pub unsafe fn draw(&self) {
        self.shader.bind();
        for (i, texture) in self.textures.iter().enumerate() {
            texture.bind_texture_unit(i as u32);
        }

        // bind the actual board state
        let game_state = self.game_state.read().unwrap();
        let vertices = Renderer::get_board_vertices(&game_state.selected_field);
        let (piece_vertices, piece_indices) = Renderer::get_piece_vertices_and_indices(
            game_state.game.back().unwrap().position_matrix().borrow(),
        );
        drop(game_state);
        self.vertex_buffer.bind();
        self.vertex_buffer
            .buffer_sub_data(&vertices, vertices.len(), 0);
        self.vertex_buffer
            .buffer_sub_data(&piece_vertices, piece_vertices.len(), 324);

        self.index_buffer.bind();
        self.index_buffer
            .buffer_sub_data(&piece_indices, piece_indices.len(), 384);

        self.vertex_array.bind();
        gl!(self.gl.draw_elements(
            GL_TRIANGLES,
            *self.index_buffer.get_index_count(),
            GL_UNSIGNED_INT,
            null_mut::<c_void>()
        ));
    }

    pub unsafe fn set_blend_func(gl: Rc<GL>) {
        gl!(gl.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA));
        gl!(gl.enable(GL_BLEND));
    }

    // TO DO: Move creation of one vertex into own method!
    pub unsafe fn get_board_vertices(selected_field: &Option<(usize, usize)>) -> Vec<f32> {
        let mut vertices = Vec::new();
        for i in 0..=8 {
            for j in 0..=8 {
                let mut texture_id: f32;

                // upper right square
                texture_id = ((i + j + 1) % 2) as f32;
                vertices.push(Vertex {
                    position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(0.0, 0.0),
                    texture_id,
                });

                // upper left square
                texture_id = ((i + j) % 2) as f32;
                vertices.push(Vertex {
                    position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(1.0, 0.0),
                    texture_id,
                });

                // lower left square
                texture_id = ((i + j + 1) % 2) as f32;
                vertices.push(Vertex {
                    position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(1.0, 1.0),
                    texture_id,
                });

                // lower right square
                texture_id = ((i + j) % 2) as f32;
                vertices.push(Vertex {
                    position: Position(i as f32 * WIDTH / 8.0, j as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(0.0, 1.0),
                    texture_id,
                });
            }
        }

        if let Some((x, y)) = *selected_field {
            let selected_field_indices = Renderer::get_board_indices_for_field(x, y);
            for index in selected_field_indices {
                vertices[index].texture_id = 3.0;
            }
        };

        Renderer::deserialize(vertices)
    }

    fn get_board_indices() -> Vec<u32> {
        let mut indices = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                indices.push(36 * i + 4 * j);
                indices.push(36 * (i + 1) + 4 * j + 1);
                indices.push(36 * (i + 1) + 4 * (j + 1) + 2);

                indices.push(36 * (i + 1) + 4 * (j + 1) + 2);
                indices.push(36 * i + 4 * (j + 1) + 3);
                indices.push(36 * i + 4 * j);
            }
        }
        indices
    }

    fn get_board_indices_for_field(x: usize, y: usize) -> Vec<usize> {
        vec![
            36 * x + 4 * y,
            36 * (x + 1) + 4 * y + 1,
            36 * (x + 1) + 4 * (y + 1) + 2,
            36 * (x + 1) + 4 * (y + 1) + 2,
            36 * x + 4 * (y + 1) + 3,
            36 * x + 4 * y,
        ]
    }

    fn deserialize(vertices: Vec<Vertex>) -> Vec<f32> {
        let mut result = Vec::new();
        for mut vertex in vertices {
            result.append(&mut vertex.deserialize());
        }
        result
    }

    unsafe fn get_piece_vertices_and_indices(
        position_matrix: Ref<PositionMatrix>,
    ) -> (Vec<f32>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        const INDEX_OFFSET: u32 = 324;

        for (i, rank) in position_matrix.0.iter().enumerate() {
            for (j, piece) in rank.iter().enumerate() {
                if piece.piecetype() == &PieceType::None {
                    continue;
                }

                let (u, v) = Renderer::get_texture_coords_from_piece(piece);
                vertices.push(Vertex {
                    position: Position(j as f32 * WIDTH / 8.0, i as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u, v),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position((j + 1) as f32 * WIDTH / 8.0, i as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u + (1.0 / 6.0), v),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position((j + 1) as f32 * WIDTH / 8.0, (i + 1) as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u + (1.0 / 6.0), v + 0.5),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position(j as f32 * WIDTH / 8.0, (i + 1) as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u, v + 0.5),
                    texture_id: 2.0,
                });

                indices.push(INDEX_OFFSET + vertices.len() as u32 - 4);
                indices.push(INDEX_OFFSET + vertices.len() as u32 - 3);
                indices.push(INDEX_OFFSET + vertices.len() as u32 - 2);
                indices.push(INDEX_OFFSET + vertices.len() as u32 - 2);
                indices.push(INDEX_OFFSET + vertices.len() as u32 - 1);
                indices.push(INDEX_OFFSET + vertices.len() as u32 - 4);
            }
        }
        (Renderer::deserialize(vertices), indices)
    }

    fn get_texture_coords_from_piece(piece: &Piece) -> (f32, f32) {
        let u: f32 = match piece.piecetype() {
            PieceType::King => 0.0,
            PieceType::Queen => 1.0 / 6.0,
            PieceType::Bishop => 2.0 / 6.0,
            PieceType::Knight => 3.0 / 6.0,
            PieceType::Rook => 4.0 / 6.0,
            PieceType::Pawn => 5.0 / 6.0,
            _ => panic!("Unknown piece type"),
        };

        let v: f32 = match piece.color() {
            Color::Black => 0.0,
            Color::White => 0.5,
            Color::None => 0.0,
        };

        (u, v)
    }

    pub unsafe fn get_clicked_field(window: *const GLFWwindow) -> (usize, usize) {
        let (xpos, ypos) = Renderer::get_cursor_position(window);
        let x = xpos / WIDTH * 8.0;
        let y = ypos / HEIGHT * 8.0;
        (x as usize, y as usize)
    }

    pub unsafe fn get_cursor_position(window: *const GLFWwindow) -> (f32, f32) {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        glfwGetCursorPos(window, &mut x, &mut y);
        (x as f32, HEIGHT - y as f32)
    }
}

pub unsafe extern "C" fn click_callback(
    window: *const GLFWwindow,
    button: c_int,
    action: c_int,
    _mods: c_int,
) {
    if action == GLFW_PRESS {
        match button {
            GLFW_MOUSE_BUTTON_LEFT => {
                let (x, y) = Renderer::get_clicked_field(window);
                toggle_field(glfwGetWindowUserPointer(window), (x, y));
            }
            GLFW_MOUSE_BUTTON_RIGHT => {
                deselected_field(glfwGetWindowUserPointer(window));
            }
            _ => {}
        }
    }
}

pub extern "C" fn window_size_callback(_window: *const GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        WIDTH = width as f32;
        HEIGHT = height as f32;
    }
}

pub extern "C" fn framebuffer_size_callback(
    _window: *const GLFWwindow,
    width: c_int,
    height: c_int,
) {
    unsafe {
        WIDTH = width as f32;
        HEIGHT = height as f32;
    }
}

pub unsafe fn toggle_field(pointer: *const c_void, value: (usize, usize)) {
    let game_state_arc = Arc::from_raw(pointer as *const RwLock<GameState>);
    let mut game_state = game_state_arc.write().unwrap();
    let return_value = match game_state.selected_field {
        Some(inner) if inner == value => None,
        Some(inner) if inner != value => {
            let mut move_string = String::new();
            move_string.push(char::from_digit(inner.1 as u32 + 1, 10).unwrap());
            move_string.push(char::from_digit(inner.0 as u32 + 1, 10).unwrap());
            move_string.push(char::from_digit(value.1 as u32 + 1, 10).unwrap());
            move_string.push(char::from_digit(value.0 as u32 + 1, 10).unwrap());
            State::perform_turn_from_input(move_string, &mut game_state.game);
            None
        }
        _ => Some(value),
    };
    game_state.selected_field = return_value;
    drop(game_state);
    forget(game_state_arc);
}

pub unsafe fn deselected_field(pointer: *const c_void) {
    let selected_field_arc = Arc::from_raw(pointer as *const RwLock<Option<(usize, usize)>>);
    *selected_field_arc.write().unwrap() = None;
    forget(selected_field_arc);
}
