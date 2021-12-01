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
*/

// TO DO: Refactor the way the renderer and game state are passed to glfw callbacks.
pub static mut WIDTH: f32 = 1024.0;
pub static mut HEIGHT: f32 = 768.0;

#[allow(dead_code)]
pub struct Renderer {
    pub gl: Rc<GL>,
    pub glfw: Glfw,
    game_state: Arc<RwLock<GameState>>,
    shader: Shader,
    v_array: VertexArray,
    v_buffer: VertexBuffer,
    i_buffer: IndexBuffer,
    textures: Vec<Texture>,
}

pub enum UiElement {
    None,
    Field(usize, usize),
    BackwardButton,
    ForwardButton,
}

pub struct Glfw {
    monitor: *mut GLFWmonitor,
    share: *mut GLFWwindow,
    window: *mut GLFWwindow,
}

impl Renderer {
    pub unsafe fn init(fen: Option<String>) -> Renderer {
        let glfw = Renderer::init_glfw().expect("Failed to initialize GLFW");
        let gl = Rc::new(GL::bind());

        // vertex buffer
        // 324 (tiles)
        // 128 (pieces)
        // 8 (UI)
        let v_buffer = VertexBuffer::new(460, Rc::clone(&gl));
        let mut layout = VertexBufferLayout::new(v_buffer.buffer_id);
        layout.push::<f32>(2);
        layout.push::<f32>(2);
        layout.push::<f32>(1);

        // we can bind the UI once at the start
        let ui_verts = Renderer::get_ui_vertices();
        v_buffer.buffer_sub_data(&ui_verts, ui_verts.len(), 452);

        // vertex array
        let mut v_array = VertexArray::new(Rc::clone(&gl));
        v_array.add_buffer(&v_buffer, &layout);

        // indices
        // 384 (tiles)
        // 192 (pieces)
        // 12 (UI)
        let board_indices = Renderer::get_board_indices();
        let i_buffer = IndexBuffer::new(588, Rc::clone(&gl));
        i_buffer.bind();
        i_buffer.buffer_sub_data(&board_indices, board_indices.len(), 0);
        let ui_indices = Renderer::get_ui_indices(452);
        i_buffer.buffer_sub_data(&ui_indices, ui_indices.len(), 576);

        // shader
        let mut shader = Shader::new(
            String::from("./src/library/gui/res/simple.shader"),
            Rc::clone(&gl),
        );

        // textures
        shader.bind();
        let white_field = Texture::new("./src/library/gui/res/img/white_field.png", Rc::clone(&gl));
        let black_field = Texture::new("./src/library/gui/res/img/black_field.png", Rc::clone(&gl));
        let pieces = Texture::new("./src/library/gui/res/img/pieces.png", Rc::clone(&gl));
        let ui_icons = Texture::new("./src/library/gui/res/img/ui_icons.png", Rc::clone(&gl));
        shader.set_uniform_1iv("u_Textures", vec![0, 1, 2, 3]);

        Renderer::set_blend_func(Rc::clone(&gl));

        // "bind" the game state to the glfw window
        let mut active_states: LinkedList<State> = LinkedList::new();
        active_states.push_back(State::new(fen));
        let game_state = Arc::new(RwLock::new(GameState {
            selected_field: None,
            active_states,
            inactive_states: LinkedList::new(),
        }));

        glfwSetWindowUserPointer(glfw.window, Arc::as_ptr(&game_state) as *const c_void);

        Renderer {
            gl,
            glfw,
            game_state,
            v_array,
            v_buffer,
            i_buffer,
            shader,
            textures: vec![white_field, black_field, pieces, ui_icons],
        }
    }

    fn init_glfw() -> Option<Glfw> {
        unsafe {
            if glfwInit() == 0 {
                return None;
            }

            let mut glfw = Glfw {
                monitor: null_mut(),
                share: null_mut(),
                window: null_mut(),
            };

            let title = CString::new("Rust chess (OpenGL)").unwrap();

            glfw.window = glfwCreateWindow(
                WIDTH as i32,
                HEIGHT as i32,
                title.as_ptr(),
                glfw.monitor,
                glfw.share,
            );

            if glfw.window.is_null() {
                glfwTerminate();
                return None;
            }

            glfwMakeContextCurrent(glfw.window);
            glfwSwapInterval(1);
            Some(glfw)
        }
    }

    pub fn get_window(&self) -> *mut GLFWwindow {
        self.glfw.window
    }

    pub fn check_game_over(&self) -> GameOver {
        let game_state = self.game_state.read().unwrap();
        let game_over = game_state.active_states.back().unwrap().check_game_over();
        drop(game_state);
        game_over
    }

    pub fn clear(&self) {
        self.v_array.unbind();
        self.v_buffer.unbind();
        self.i_buffer.unbind();
        self.shader.unbind();
        unsafe {
            gl!(self.gl.clear(GL_COLOR_BUFFER_BIT));
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            self.set_view();
        }
        self.bind_textures();
        self.bind_board_state();
        self.draw_call();
    }

    pub unsafe fn get_board_transformation() -> Matrix4<f32> {
        translate(0.1, 0.1, 0.0) * scale(0.6, 0.8, 1.0)
    }
    unsafe fn set_view(&mut self) {
        let mvp = ortho(0.0, 1.0, 0.0, 1.0, -0.5, 0.5) * Renderer::get_board_transformation();
        self.shader.bind();
        self.shader.set_uniform_mat4f("u_MVP", mvp);
        self.shader
            .set_uniform_mat4f("u_MVP_UI", ortho(0.0, 1.0, 0.0, 1.0, -0.5, 0.5));
        gl!(self.gl.viewport(0, 0, WIDTH as i32, HEIGHT as i32));
    }

    fn draw_call(&self) {
        unsafe {
            self.v_array.bind();
            gl!(self.gl.draw_elements(
                GL_TRIANGLES,
                *self.i_buffer.get_index_count(),
                GL_UNSIGNED_INT,
                null_mut::<c_void>()
            ));
        }
    }

    fn bind_board_state(&self) {
        self.v_buffer.bind();
        let game_state = self.game_state.read().unwrap();
        let b_verts = Renderer::get_board_vertices(&game_state.selected_field);
        self.v_buffer.buffer_sub_data(&b_verts, b_verts.len(), 0);
        unsafe {
            let pos_matrix = game_state
                .active_states
                .back()
                .unwrap()
                .position_matrix()
                .borrow();
            let (p_verts, p_inds) = Renderer::get_piece_vertices_and_indices(pos_matrix);
            self.v_buffer.buffer_sub_data(&p_verts, p_verts.len(), 324);
            self.i_buffer.bind();
            self.i_buffer.buffer_sub_data(&p_inds, p_inds.len(), 384);
        }
    }

    fn bind_textures(&self) {
        for (i, texture) in self.textures.iter().enumerate() {
            texture.bind_texture_unit(i as u32);
        }
    }

    pub unsafe fn set_blend_func(gl: Rc<GL>) {
        gl!(gl.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA));
        gl!(gl.enable(GL_BLEND));
    }

    pub fn get_board_vertices(selected_field: &Option<(usize, usize)>) -> Vec<f32> {
        let mut vertices = Vec::new();
        for i in 0..=8 {
            for j in 0..=8 {
                for k in 1..=4 {
                    vertices.push(Renderer::get_vertex(i, j, k));
                }
            }
        }

        if let Some((x, y)) = *selected_field {
            let selected_field_indices = Renderer::get_board_indices_for_field(x, y);
            for index in selected_field_indices {
                vertices[index].texture_id = 9.0;
            }
        };

        Renderer::deserialize(vertices)
    }

    #[rustfmt::skip]
    pub fn get_ui_vertices() -> Vec<f32> {
        // one button should be 8x8% of the screen
        let base_position = (0.75, 0.42);
        let mut vertices = Vec::new();
        for i in 0..=1 {
            for j in 1..=4 {
                let position = match j {
                    1 => Position(base_position.0 + 0.12 * i as f32,        base_position.1),
                    2 => Position(base_position.0 + 0.12 * i as f32 + 0.08, base_position.1),
                    3 => Position(base_position.0 + 0.12 * i as f32 + 0.08, base_position.1 + 0.16),
                    4 => Position(base_position.0 + 0.12 * i as f32,        base_position.1 + 0.16),
                    _ => panic!("Invalid corner index"),
                };
                let texture_coords = Renderer::get_texture_coords(j, 2., i as f32, 1., 0.);
                vertices.push(Vertex {
                    position,
                    texture_coords,
                    texture_id: 3.0,
                });
            }
        }

        Renderer::deserialize(vertices)
    }

    #[rustfmt::skip]
    fn get_texture_coords(corner: i32, num_cols: f32, col_idx: f32, num_rows: f32, row_idx: f32) -> TextureCoords {
        match corner {
            1 => TextureCoords(col_idx      / num_cols, row_idx     / num_rows),
            2 => TextureCoords((col_idx+1.) / num_cols, row_idx     / num_rows),
            3 => TextureCoords((col_idx+1.) / num_cols, (row_idx+1.) / num_rows),
            4 => TextureCoords(col_idx      / num_cols, (row_idx+1.) / num_rows),
            _ => panic!("Invalid corner index"),
        }
    }

    fn get_vertex(rank: i32, file: i32, corner: i32) -> Vertex {
        let position = Position(rank as f32 * 1.0 / 8.0, file as f32 * 1.0 / 8.0);

        let texture_coords = Renderer::get_texture_coords(corner, 1., 0., 1., 0.);

        let texture_id = match corner {
            1 | 3 => ((rank + file + 1) % 2) as f32,
            2 | 4 => ((rank + file) % 2) as f32,
            _ => panic!("Invalid corner index"),
        };

        Vertex {
            position,
            texture_coords,
            texture_id,
        }
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

    fn get_ui_indices(offset: u32) -> Vec<u32> {
        vec![
            offset,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 3,
            offset,
            offset + 4,
            offset + 5,
            offset + 6,
            offset + 6,
            offset + 7,
            offset + 4,
        ]
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
                    position: Position(j as f32 * 1.0 / 8.0, i as f32 * 1.0 / 8.0),
                    texture_coords: TextureCoords(u, v),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position((j + 1) as f32 * 1.0 / 8.0, i as f32 * 1.0 / 8.0),
                    texture_coords: TextureCoords(u + (1.0 / 6.0), v),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position((j + 1) as f32 * 1.0 / 8.0, (i + 1) as f32 * 1.0 / 8.0),
                    texture_coords: TextureCoords(u + (1.0 / 6.0), v + 0.5),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position(j as f32 * 1.0 / 8.0, (i + 1) as f32 * 1.0 / 8.0),
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

    pub unsafe fn get_clicked_element(window: *const GLFWwindow) -> UiElement {
        let (xpos, ypos) = Renderer::get_cursor_position(window); // WIDTH x HEIGHT
        let view_click_position = Vector4::new(xpos / WIDTH, ypos / HEIGHT, 0.0, 1.0);
        let mut board_transformation = Renderer::get_board_transformation();
        if !board_transformation.try_inverse_mut() {
            panic!("Could not invert board transformation")
        };
        let board_click_position = board_transformation * view_click_position;

        if board_click_position.x >= 0.
            && board_click_position.x <= 1.
            && board_click_position.y >= 0.
            && board_click_position.y <= 1.
        {
            return UiElement::Field(
                (board_click_position.x * 8.0) as usize,
                (board_click_position.y * 8.0) as usize,
            );
        }

        if view_click_position.y < 0.42 || view_click_position.y > 0.58 {
            return UiElement::None;
        }

        if view_click_position.x >= 0.75 && view_click_position.x <= 0.83 {
            return UiElement::BackwardButton;
        }

        if view_click_position.x >= 0.87 && view_click_position.x <= 0.95 {
            return UiElement::ForwardButton;
        }

        UiElement::None
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
    if action != GLFW_PRESS {
        return;
    }
    match button {
        GLFW_MOUSE_BUTTON_LEFT => {
            let ui_element = Renderer::get_clicked_element(window);
            match ui_element {
                UiElement::Field(x, y) => {
                    toggle_field(glfwGetWindowUserPointer(window), (x, y));
                }
                UiElement::BackwardButton | UiElement::ForwardButton => {
                    scroll_state(glfwGetWindowUserPointer(window), ui_element);
                }
                _ => {}
            }
        }
        GLFW_MOUSE_BUTTON_RIGHT => {
            deselected_field(glfwGetWindowUserPointer(window));
        }
        _ => {}
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

pub unsafe fn scroll_state(game_state_ptr: *const c_void, button: UiElement) {
    let game_state_arc = Arc::from_raw(game_state_ptr as *const RwLock<GameState>);
    let mut game_state = game_state_arc.write().unwrap();
    match button {
        UiElement::BackwardButton => {
            if game_state.active_states.len() > 1 {
                let last_active_state = game_state.active_states.pop_back().unwrap();
                game_state.inactive_states.push_back(last_active_state);
            }
        }
        UiElement::ForwardButton => {
            if game_state.inactive_states.len() > 0 {
                let last_inactive_state = game_state.inactive_states.pop_back().unwrap();
                game_state.active_states.push_back(last_inactive_state);
            }
        }
        _ => panic!("There should only be forward and backward buttons at this point."),
    };
    drop(game_state);
    forget(game_state_arc);
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
            let current_state = game_state.active_states.back().unwrap();
            let new_state = State::perform_turn_from_input(move_string, current_state);
            drop(current_state);
            handle_state(new_state, &mut game_state);
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
