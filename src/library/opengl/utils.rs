use crate::library::opengl::opengl::*;
use std::ffi::{CStr};

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

#[cfg(debug_assertions)]
pub unsafe fn gl_clear_errors() {
    loop {
        if glGetError() == GL_NO_ERROR {
            return
        }
    }
}

#[cfg(debug_assertions)]
pub unsafe fn gl_print_errors(file: &str, line: u32) {
    loop {
        let error = glGetError();
        match error {
            GL_NO_ERROR => return,
            GL_INVALID_ENUM      => panic!("OpenGL-Error: Invalid enum.      ({}:{})", file, line),
            GL_INVALID_VALUE     => panic!("OpenGL-Error: Invalid value.     ({}:{})", file, line),
            GL_INVALID_OPERATION => panic!("OpenGL-Error: Invalid operation. ({}:{})", file, line),
            GL_OUT_OF_MEMORY     => panic!("OpenGL-Error: Out of memory.     ({}:{})", file, line),
            _ => panic!("OpenGL-Error: {:?}. ({}:{})", error, file, line)
        };
    }
}

pub unsafe fn print_opengl_version(gl: &GL) {
    println!("{:?}", CStr::from_ptr(gl.get_string(GL_VERSION) as *const i8));
}

pub unsafe fn print_opengl_extensions(gl: &GL) {
    let mut i: GLint = 0;
    gl.get_integerv(GL_NUM_EXTENSIONS, &mut i as *mut GLint);    
    for index in 0..i {
        let ptr = gl.get_stringi(GL_EXTENSIONS, index as u32);
        if ptr.is_null() {
            println!("Error on glGetStringi. Function pointer: {:?}", gl.glGetStringi);
        } else {
            println!("{:?}", CStr::from_ptr(ptr  as *const i8));
        }
    }
}