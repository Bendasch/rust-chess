use crate::library::glfw::*;
use crate::library::opengl::opengl::*;
use std::ffi::{CString, CStr};
use std::ptr::{null_mut};
use std::mem::size_of;
use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use libc::{c_int, c_uint, c_char, c_uchar, c_float, c_void};

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! gl {
    ($e:expr) => {
        gl_clear_errors(); 
        $e; gl_print_errors(file!(), line!());
    };
    ($s:stmt) => {        
        gl_clear_errors(); 
        $s gl_print_errors(file!(), line!());
    }
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! gl {
    ($e:expr) => {$e;};
    ($s:stmt) => {$s};
}

unsafe fn compile_shader(_type: GLenum, source: CString, gl: &GL) -> GLuint {
    
    //let id = _glCreateShader()(_type);
    let id = gl.create_shader(_type);
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

unsafe fn create_shader(vertex_shader: CString, fragment_shader: CString, gl: &GL) -> GLuint {

    let program = _glCreateProgram()();    
    let vertex_shader = compile_shader(GL_VERTEX_SHADER, vertex_shader, gl);
    let fragment_shader = compile_shader(GL_FRAGMENT_SHADER, fragment_shader, gl);
    
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

        let gl: GL = GL::bind();

        println!("{:?}", CStr::from_ptr(glGetString(GL_VERSION) as *const i8));
        
        glfwSwapInterval(1);

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
        
        let positions: [c_float; 8] = [
            -0.5,  -0.5, 
             0.5,  -0.5, 
             0.5,   0.5,
            -0.5,   0.5,
        ];
        
        //let glGenBuffers = _glGenBuffers();
        let gl_bind_buffer = _glBindBuffer();
        let gl_buffer_data = _glBufferData();

        let mut buffer: c_uint = 0;
        gl.gen_buffers(1, &mut buffer);
        gl_bind_buffer(GL_ARRAY_BUFFER, buffer);
        gl_buffer_data(GL_ARRAY_BUFFER, (4 * 2 * size_of::<c_float>()) as GLsizeiptr, positions.as_ptr() as *const c_void, GL_STATIC_DRAW);
        
        let indices: [c_uint; 6] = [
            0, 1, 2,
            2, 3, 0
        ];
            
        let mut ibo: c_uint = 0;
        gl.gen_buffers(1, &mut ibo);
        gl_bind_buffer(GL_ELEMENT_ARRAY_BUFFER, ibo);
        gl_buffer_data(GL_ELEMENT_ARRAY_BUFFER, (2 * 3 * size_of::<c_uint>()) as GLsizeiptr, indices.as_ptr() as *const c_void, GL_STATIC_DRAW);
        //gl_bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        
        _glEnableVertexAttribArray()(0);
        _glVertexAttribPointer()(0, 2, GL_FLOAT, GL_FALSE, 2 * size_of::<c_float>() as i32, 0 as *mut c_void);

        let (vertex_shader, fragment_shader) = read_shaders_from_file();
        let shader: GLuint = create_shader(vertex_shader, fragment_shader, &gl);
        gl!(_glUseProgram()(shader));       
        
        let u_color = CString::new("u_Color").unwrap();
        gl!(let location = _glGetUniformLocation()(shader, u_color.as_ptr() as *const GLchar));
        let mut red = 0.5f32;
        let mut red_increment = 0.005f32;
        let mut green = 0.25f32;
        let mut green_increment = 0.001f32;
        let mut blue = 0.65f32;
        let mut blue_increment = 0.01f32;

        while glfwWindowShouldClose(window) == 0 {
            
            glClear(GL_COLOR_BUFFER_BIT);
                        
            gl!(glUniform4f(location, red, green, blue, 0.9f32));
            gl!(glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *mut c_void));

            if red > 0.9 || red < 0.1 {
                red_increment *= -1.0f32; 
            }

            if green > 0.7 || green < 0.3 {
                green_increment *= -1.0f32;
            }

            if blue > 0.95 || blue < 0.05 {
                blue_increment *= -1.0f32;
            }

            red += red_increment;
            green += green_increment;
            blue += blue_increment;

            glfwSwapBuffers(window);
            
            glfwPollEvents();
        }
        
        _glDeleteProgram()(shader);
        glfwTerminate();
    }
}

#[cfg(debug_assertions)]
unsafe fn gl_clear_errors() {
    loop {
        if glGetError() == GL_NO_ERROR {
            return
        }
    }
}

#[cfg(debug_assertions)]
unsafe fn gl_print_errors(file: &str, line: u32) {
    loop {
        let error = glGetError();
        match error {
            GL_NO_ERROR => return,
            GL_INVALID_ENUM      => println!("OpenGL-Error: Invalid enum.      ({}:{})", file, line),
            GL_INVALID_VALUE     => println!("OpenGL-Error: Invalid value.     ({}:{})", file, line),
            GL_INVALID_OPERATION => println!("OpenGL-Error: Invalid operation. ({}:{})", file, line),
            GL_OUT_OF_MEMORY     => println!("OpenGL-Error: Out of memory.     ({}:{})", file, line),
            _ => println!("OpenGL-Error: {:?}. ({}:{})", error, file, line)
        };
        
        if error == GL_NO_ERROR {
            return;
        }
    }
}

enum ShaderType {
    None,
    Vertex,
    Fragment
}

fn read_shaders_from_file() -> (CString, CString) {
    
    use ShaderType::*;

    let (mut vertex_shader, mut fragment_shader) = (String::new(), String::new());
    
    let shader_file_name: &str = "./src/library/opengl/simple.shader";
    let mut file = File::open(shader_file_name).expect(format!("Couldn't read shader file {}", shader_file_name).as_str());
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Couldn't read contents of file {}", shader_file_name).as_str());
    let lines: Vec<&str> = contents.split("\n").collect(); 
    
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