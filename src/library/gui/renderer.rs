use crate::gl;
use crate::library::game::*;
use crate::library::gui::{
    glfw::*,
    opengl::*,
    index_buffer::*,
    vertex_buffer::*,
    vertex_buffer_layout::*,
    vertex_array::*,
    shader::*,
    utils::*,
    gl_maths::*,
    texture::*
};
use std::{
    ffi::{CString},
    ptr::{null_mut},
    rc::Rc,
    sync::Mutex
};
use libc::{c_void, c_int};

pub static WIDTH: f32 = 1024.0;
pub static HEIGHT: f32 = 768.0;
//static SELECTED_FIELD: Mutex<Option<(usize, usize)>> = Mutex::new(None);

#[allow(dead_code)]
pub struct Renderer {
    pub gl: Rc<GL>,
    pub window: *mut GLFWwindow,
    shader: Shader,
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    textures: Vec<Texture>,
    selected_field: Mutex<Option<(u32, u32)>>
}

impl Renderer {
    
    pub unsafe fn init() -> Renderer  {
        
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
        
        let vertex_buffer = VertexBuffer::new(null_mut() as *const c_void, 452, Rc::clone(&gl));
        let mut layout = VertexBufferLayout::new(vertex_buffer.buffer_id);
        layout.push::<f32>(2);
        layout.push::<f32>(2);
        layout.push::<f32>(1);
        
        let mut vertex_array = VertexArray::new(Rc::clone(&gl));
        vertex_array.add_buffer(&vertex_buffer, &layout);
        
        let vertices = Renderer::get_board_vertices();
        vertex_buffer.bind();
        vertex_buffer.buffer_sub_data(vertices.as_ptr() as *const c_void, vertices.len(), 0);
        
        let board_indices = Renderer::get_board_indices();
        let index_buffer = IndexBuffer::new(null_mut() as *const c_void, 576, Rc::clone(&gl));
        index_buffer.bind();
        index_buffer.buffer_sub_data(board_indices.as_ptr() as *const c_void, board_indices.len(), 0);
        
        let mut shader = Shader::new(String::from("./src/library/gui/simple.shader"), Rc::clone(&gl));
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
        
        Renderer { 
            gl, 
            window,
            vertex_array,
            vertex_buffer,
            index_buffer,
            shader, 
            textures: vec![white_field, black_field, pieces],
            selected_field: Mutex::new(None)
        }
    }
    
    pub unsafe fn clear(&self) {
        gl!(self.gl.clear(GL_COLOR_BUFFER_BIT));
    }
    
    pub unsafe fn update(&mut self) {    
        let mvp = ortho(0.0, WIDTH, 0.0, HEIGHT, -0.5, 0.5);
        self.shader.bind();
        self.shader.set_uniform_mat4f("u_MVP", mvp);
    }
    
    pub unsafe fn draw(&self, position_matrix: Ref<PositionMatrix>) {
        
        self.shader.bind();
        for (i, texture) in self.textures.iter().enumerate() {
            texture.bind_texture_unit(i as u32);
        }
        
        // bind the actual board state
        let (piece_vertices, piece_indices) = Renderer::get_piece_vertices_and_indices(position_matrix);
        self.vertex_buffer.bind();
        self.vertex_buffer.buffer_sub_data(piece_vertices.as_ptr() as *const c_void, piece_vertices.len(), 324);
        self.index_buffer.bind();
        self.index_buffer.buffer_sub_data(piece_indices.as_ptr() as *const c_void, piece_indices.len(), 384);
        
        self.vertex_array.bind();    
        gl!(self.gl.draw_elements(GL_TRIANGLES, *self.index_buffer.get_index_count(), GL_UNSIGNED_INT, 0 as *mut c_void));
    }

    pub unsafe fn set_blend_func(gl: Rc<GL>) {
        gl!(gl.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA));
        gl!(gl.enable(GL_BLEND));
    }

    pub fn get_board_vertices() -> Vec<f32> {
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

        Renderer::deserialize(vertices)
    }

    fn get_board_indices() -> Vec<u32> {
        let mut indices = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                indices.push(36*i       + 4*j);
                indices.push(36*(i+1)   + 4*j     + 1);
                indices.push(36*(i+1)   + 4*(j+1) + 2);

                indices.push(36*(i+1)   + 4*(j+1) + 2);
                indices.push(36*i       + 4*(j+1) + 3);
                indices.push(36*i       + 4*j); 
            }
        }
        indices
    }

    fn deserialize(vertices: Vec<Vertex>) -> Vec<f32> {
        let mut result = Vec::new();
        for mut vertex in vertices {
            result.append(&mut vertex.deserialize());
        }
        result
    }

    fn get_piece_vertices_and_indices(position_matrix: Ref<PositionMatrix>) -> (Vec<f32>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        const INDEX_OFFSET: u32 = 324;

        for (i, rank) in position_matrix.0.iter().enumerate() {
            for (j, piece) in rank.iter().enumerate() {

                if piece.piecetype() == &PieceType::None { continue; }

                let (u, v) = Renderer::get_texture_coords_from_piece(&piece);
                vertices.push(Vertex {
                    position: Position(j as f32 * WIDTH / 8.0, i as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u, v),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position((j+1) as f32 * WIDTH / 8.0, i as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u+(1.0/6.0), v),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position((j+1) as f32 * WIDTH / 8.0, (i+1) as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u+(1.0/6.0), v+0.5),
                    texture_id: 2.0,
                });
                vertices.push(Vertex {
                    position: Position(j as f32 * WIDTH / 8.0, (i+1) as f32 * HEIGHT / 8.0),
                    texture_coords: TextureCoords(u, v+0.5),
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
            PieceType::King     => 0.0,
            PieceType::Queen    => 1.0 / 6.0,
            PieceType::Bishop   => 2.0 / 6.0,
            PieceType::Knight   => 3.0 / 6.0,
            PieceType::Rook     => 4.0 / 6.0,
            PieceType::Pawn     => 5.0 / 6.0,
            _ => panic!("Unknown piece type")
        };

        let v: f32 = match piece.color() {
            Color::Black => 0.0,
            Color::White => 0.5,
            Color::None => 0.0,
        };
        
        (u, v)
    }
    
    pub fn get_clicked_field(window: *const GLFWwindow) -> (usize, usize) {
        let (xpos, ypos) = unsafe{Renderer::get_cursor_position(window)};
        let x = xpos / WIDTH * 8.0;
        let y = ypos / HEIGHT * 8.0;
        (x as usize, y as usize)
    }
    
    pub unsafe fn get_cursor_position(window: *const GLFWwindow) -> (f32, f32) {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        glfwGetCursorPos(window, &mut x, &mut y);
        (x as f32, HEIGHT-y as f32)
    }
}

pub extern fn callback(window: *const GLFWwindow, button: c_int, action: c_int, _mods: c_int) {
    if button == GLFW_MOUSE_BUTTON_LEFT && action == GLFW_PRESS {
        let (x, y) = Renderer::get_clicked_field(window);
        println!("x: {}, y: {}", x, y);
        /*
        let mut selected_field = SELECTED_FIELD.lock().unwrap(); 
        *selected_field = Some((x,y));
        */
    }
}