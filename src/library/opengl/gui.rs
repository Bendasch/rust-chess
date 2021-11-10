use crate::gl;
use crate::library::opengl::renderer::*;
use crate::library::glfw::*;
use crate::library::opengl::opengl::*;
use crate::library::opengl::vertex_buffer::*;
use crate::library::opengl::index_buffer::*;
use crate::library::opengl::vertex_array::*;
use crate::library::opengl::vertex_buffer_layout::*;
use std::ffi::{CString, CStr};
use std::ptr::{null_mut};
use std::mem::size_of;
use std::fs::File;
use std::io::Read;
#[allow(unused_imports)]
use libc::{c_int, c_uint, c_char, c_uchar, c_float, c_void};


unsafe fn compile_shader(_type: GLenum, source: CString, gl: &GL) -> GLuint {
    
    gl!(let id = gl.create_shader(_type));
    let src: *const c_char = source.as_ptr();
    let ptr: *const *const c_char = &src;
    
    gl!(gl.shader_source(id, 1, ptr, null_mut()));
    gl!(gl.compile_shader(id));

    let mut result: GLint = 0;
    gl!(gl.get_shaderiv(id, GL_COMPILE_STATUS, &mut result));

    if result as i8 == GL_FALSE {
        let mut length: GLint = 0;
        let mut message: [GLchar; 1024] = [0; 1024];
        let msg_pointer: *mut GLchar = &mut message[0];
        gl!(gl.get_shader_infolog(id, 1024, &mut length, msg_pointer));        
        match _type {
            GL_VERTEX_SHADER => println!("Vertex shader failed."),
            GL_FRAGMENT_SHADER =>  println!("Fragment shader failed."),
            _ => println!("Other shader failed...")
        };
        println!("Error: {:?}", CStr::from_ptr(msg_pointer).to_str());
        return 0;
    }

    return id;
}

unsafe fn create_shader(vertex_shader: CString, fragment_shader: CString, gl: &GL) -> GLuint {

    gl!(let program = gl.create_program());    
    let vertex_shader = compile_shader(GL_VERTEX_SHADER, vertex_shader, gl);
    let fragment_shader = compile_shader(GL_FRAGMENT_SHADER, fragment_shader, gl);
    
    gl!(gl.attach_shader(program, vertex_shader));   
    gl!(gl.attach_shader(program, fragment_shader));   
    
    gl!(gl.link_program(program));
    gl!(gl.validate_program(program));
    
    gl!(gl.delete_shader(vertex_shader));
    gl!(gl.delete_shader(fragment_shader));
    
    return program;
}

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
    
    let (vertex_shader, fragment_shader) = read_shaders_from_file();
    let shader: GLuint = create_shader(vertex_shader, fragment_shader, &gl);
    gl!(gl.use_program(shader));       
    
    let u_color = CString::new("u_Color").unwrap();
    gl!(let location = gl.get_uniform_location(shader, u_color.as_ptr() as *const GLchar));

    gl!(gl.bind_vertex_array(0));
    gl!(gl.use_program(0));
    gl!(gl.bind_buffer(GL_ARRAY_BUFFER, 0));
    gl!(gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0));

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


        gl!(gl.clear(GL_COLOR_BUFFER_BIT));
                    
        gl!(gl.use_program(shader));
        gl!(gl.uniform_4f(location, red, green, blue, 0.9f32));
        
        vertex_array.bind();

        gl!(gl.draw_elements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *mut c_void));


        glfwSwapBuffers(window);
        
        glfwPollEvents();
    }
    
    gl!(gl.delete_program(shader));
    drop(vertex_buffer);
    drop(index_buffer);
    
    glfwTerminate();
}

enum ShaderType {
    None,
    Vertex,
    Fragment
}

fn read_shaders_from_file() -> (CString, CString) {
    
    let shader_file_name: &str = "./src/library/opengl/simple.shader";
    let mut file = File::open(shader_file_name).expect(format!("Couldn't read shader file {}", shader_file_name).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Couldn't read contents of file {}", shader_file_name).as_str());
    let lines: Vec<&str> = contents.split("\n").collect(); 
    
    let (mut vertex_shader, mut fragment_shader) = (String::new(), String::new());
    use ShaderType::*;
    let mut mode: ShaderType = None;

    for line in lines {
        if line.trim().starts_with("#shader") {
            if line.trim() == "#shader vertex" {
                mode = Vertex;
            } else if line.trim() == "#shader fragment" {
                mode = Fragment;
            }
            continue;
        }
        match mode {
            Vertex => { vertex_shader.push_str(line); vertex_shader.push_str("\n"); },
            Fragment => { fragment_shader.push_str(line); fragment_shader.push_str("\n"); },
            None => continue
        }
    }

    let vertex_shader = CString::new(vertex_shader).unwrap();
    let fragment_shader = CString::new(fragment_shader).unwrap();

    (vertex_shader, fragment_shader)
}

#[cfg(test)]
pub mod tests {

    use super::read_shaders_from_file;

    #[test]
    fn read_shaders_from_file_vertex() {
        let (vertex, _) = read_shaders_from_file();
        let vertex_string = String::from(vertex.to_str().unwrap());
        let line_vec: Vec<&str> = vertex_string.split("\n").collect();
        assert_eq!(line_vec[0].trim(), "#version 330 core");
    }
    
    #[test]
    fn read_shaders_from_file_fragment() {
        let (_, fragment) = read_shaders_from_file();
        let fragment_string = String::from(fragment.to_str().unwrap());
        let line_vec: Vec<&str> = fragment_string.split("\n").collect();
        assert_eq!(line_vec[0].trim(), "#version 330 core");
    }
}