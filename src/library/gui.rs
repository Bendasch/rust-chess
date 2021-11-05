use crate::library::glfw::*;
use crate::library::legacy_opengl::*;
use crate::library::modern_opengl::*;
use std::ffi::{CString, CStr};
use std::ptr::{null_mut};
use std::mem::size_of;

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

        let vertex_shader = CString::new(format!("{}{}{}{}{}{}{}{}{}",
            "#version 330 core\n", 
            "\n",
            "layout(location = 0) in vec4 position;\n", 
            "\n", 
            "void main()\n",
            "{\n",
            "\n",
            "   gl_Position = position;\n",
            "}\n"
        )).expect("Vertex shader code invalid CString.");
        
        let fragment_shader = CString::new(format!("{}{}{}{}{}{}{}{}{}",
            "#version 330 core\n", 
            "\n",
            "layout(location = 0) out vec4 color;\n", 
            "\n", 
            "void main()\n",
            "{\n",
            "\n",
            "   color = vec4(1.0, 0.0, 0.0, 1.0);\n",
            "}\n"
        )).expect("Fragment shader code invalid CString.");

        let shader: GLuint = create_shader(vertex_shader, fragment_shader);
        _glUseProgram()(shader);       

        let gl_draw_arrays = _glDrawArrays();
        
        while glfwWindowShouldClose(window) == 0 {
            
            glClear(GL_COLOR_BUFFER_BIT);
            
            gl_draw_arrays(GL_TRIANGLES, 0, 3);
            
            glfwSwapBuffers(window);
            
            glfwPollEvents();
        }
        
        glfwTerminate();
    }
}