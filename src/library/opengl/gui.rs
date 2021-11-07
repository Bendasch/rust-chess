use crate::library::glfw::*;
use crate::library::opengl::legacy_opengl::*;
use crate::library::opengl::modern_opengl::*;
use std::ffi::{CString, CStr};
use std::ptr::{null_mut};
use std::mem::size_of;
use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use libc::{c_int, c_uint, c_char, c_uchar, c_float, c_void};

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

unsafe fn compile_shader(_type: GLenum, source: CString) -> GLuint {
    
    let id = _glCreateShader()(_type);
    let src: *const c_char = source.as_ptr();
    let ptr: *const *const c_char = &src;
    
    _glShaderSource()(id, 1, ptr, null_mut());
    _glCompileShader()(id);

    let mut result: GLint = 0;
    _glGetShaderiv()(id, GL_COMPILE_STATUS, &mut result);
    if result as i8 == GL_FALSE {
        let mut length: GLint = 0;
        //_glGetShaderiv()(id, GL_INFO_LOG_LENGTH, &mut length);
        let mut message: [GLchar; 1024] = [0; 1024];
        let msg_pointer: *mut GLchar = &mut message[0];
        _glGetShaderInfoLog()(id, 1024, &mut length, msg_pointer);        
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

unsafe fn create_shader(vertex_shader: CString, fragment_shader: CString) -> GLuint {

    let program = _glCreateProgram()();    
    let vertex_shader = compile_shader(GL_VERTEX_SHADER, vertex_shader);
    let fragment_shader = compile_shader(GL_FRAGMENT_SHADER, fragment_shader);
    
    _glAttachShader()(program, vertex_shader);   
    _glAttachShader()(program, fragment_shader);   
    
    _glLinkProgram()(program);
    _glValidateProgram()(program);
    
    _glDeleteShader()(vertex_shader);
    _glDeleteShader()(fragment_shader);
    
    return program;
}

pub fn run() {

    unsafe {

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

        println!("{:?}", CStr::from_ptr(glGetString(GL_VERSION) as *const i8));
        
        /*
        let mut i: GLint = 0;
        glGetIntegerv(GL_NUM_EXTENSIONS, &mut i as *mut GLint);
        println!("{:?}", i);
        
        for index in 0..i {
            let gl_get_stringi = _glGetStringi();
            let ptr = gl_get_stringi(GL_EXTENSIONS, index as u32);
            if ptr.is_null() {
                println!("Error on glGetStringi. Function pointer: {:?}", gl_get_stringi);
            } else {
                println!("{:?}", CStr::from_ptr(ptr  as *const i8));
            }
        }
        */
        
        
        let positions: [c_float; 6] = [-0.5, -0.5, 0.0,  0.5, 0.5, -0.5];
        let mut buffer: c_uint = 0;
        _glGenBuffers()(1, &mut buffer);
        _glBindBuffer()(GL_ARRAY_BUFFER, buffer);
        _glBufferData()(GL_ARRAY_BUFFER, (6 * size_of::<c_float>()) as GLsizeiptr, positions.as_ptr() as *const c_void, GL_STATIC_DRAW);
        _glEnableVertexAttribArray()(0);
        _glVertexAttribPointer()(0, 2, GL_FLOAT, GL_FALSE, 2 * size_of::<c_float>() as i32, 0 as *mut c_void);
        
        _glBindBuffer()(GL_ARRAY_BUFFER, 0);

        let (vertex_shader, fragment_shader) = read_shaders_from_file();
        let shader: GLuint = create_shader(vertex_shader, fragment_shader);
        _glUseProgram()(shader);       
                
//        let gl_draw_arrays = _glDrawArrays();

        while glfwWindowShouldClose(window) == 0 {
            
            glClear(GL_COLOR_BUFFER_BIT);
            //gl_draw_arrays(GL_TRIANGLES, 0, 3);
            glDrawArrays(GL_TRIANGLES, 0, 3);
            
            glfwSwapBuffers(window);
            
            glfwPollEvents();
        }
        
        glfwTerminate();
    }
}

fn read_shaders_from_file() -> (CString, CString) {
    
    let (mut vertex, mut fragment) = (String::new(), String::new());
    
    let shader_file_name: &str = "./src/library/opengl/simple.shader";
    let mut file = File::open(shader_file_name).expect(format!("Couldn't read shader file {}", shader_file_name).as_str());
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Couldn't read contents of file {}", shader_file_name).as_str());
    let lines: Vec<&str> = contents.split("\n").collect(); 
    
    let mut mode = "___";
    for line in lines {
        if line.trim().starts_with("#shader") {
            if line.trim() == "#shader vertex" {
                mode = "ver";
            } else if line.trim() == "#shader fragment" {
                mode = "fra";
            }
            continue;
        }
        match mode {
            "ver" => { vertex.push_str(line); vertex.push_str("\n"); },
            "fra" => { fragment.push_str(line); fragment.push_str("\n"); },
            _ => continue
        }
    }

    let vertex = CString::new(vertex).unwrap();
    let fragment = CString::new(fragment).unwrap();

    (vertex, fragment)
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