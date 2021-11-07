#[allow(unused_imports)]
use libc::{c_int, c_uint, c_char, c_uchar, c_float, c_void};
use std::ffi::{CString};
use std::mem::transmute;

use crate::library::opengl::legacy_opengl::*;


#[link(name = "Opengl32")]
extern "C" {
    pub fn wglGetProcAddress(unnamedParam1: *const c_char) -> *const ();
    pub fn wglGetCurrentContext() -> *const c_char;
}

/* 
    CONSTANTS
*/
pub const GL_EXTENSIONS: GLenum = 0x1F03;
pub const GL_ARRAY_BUFFER: GLenum = 0x8892;    
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;

pub const GL_STREAM_DRAW: GLenum = 0x88E0;
pub const GL_STREAM_READ: GLenum = 0x88E1;
pub const GL_STREAM_COPY: GLenum = 0x88E2;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_STATIC_READ: GLenum = 0x88E5;
pub const GL_STATIC_COPY: GLenum = 0x88E6;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_DYNAMIC_READ: GLenum = 0x88E9; 
pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;      

pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;

pub const GL_DELETE_STATUS: GLenum = 0x8B80;
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;

pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;

/*
    FUNCTION POINTERS
*/
#[allow(non_snake_case)]
pub unsafe fn _glGenBuffers() -> fn(n: GLsizei, buffers: *mut GLuint) {
    let function_name = CString::new("glGenBuffers").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(n: GLsizei, buffers: *mut GLuint)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glGetStringi() -> fn(name: GLenum, index: GLuint) -> *const GLubyte {
    let function_name = CString::new("glGetStringi").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(name: GLenum, index: GLuint) -> *const GLubyte> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glBindBuffer() -> fn(target: GLenum, buffer: GLuint) {
    let function_name = CString::new("glBindBuffer").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(target: GLenum, buffer: GLuint)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glBufferData() -> fn(
    target: GLenum, 
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum
) {
    let function_name = CString::new("glBufferData").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glDrawArrays() -> fn(mode: GLenum, first: GLint, count: GLsizei) {
    let function_name = CString::new("glDrawArrays").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(mode: GLenum, first: GLint, count: GLsizei)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glDrawElements() -> fn(mode: GLenum, count: GLsizei, _type: GLenum, indices: *const GLvoid) {
    let function_name = CString::new("glDrawElements").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(mode: GLenum, count: GLsizei, _type: GLenum, indices: *const GLvoid)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glEnableVertexAttribArray() -> fn(index: GLuint) {
    let function_name = CString::new("glEnableVertexAttribArray").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(index: GLuint)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glVertexAttribPointer() -> fn(
    index: GLuint, 
    size: GLint, 
    _type: GLenum, 
    normalized: GLboolean, 
    stride: GLsizei, 
    function_pointer: *const GLvoid
) {
    let function_name = CString::new("glVertexAttribPointer").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(index: GLuint, size: GLint, _type: GLenum, normalized: GLboolean, stride: GLsizei, function_pointer: *const GLvoid)> (function_pointer)   
}


#[allow(non_snake_case)]
pub unsafe fn _glCreateProgram() -> fn() -> GLuint{
    let function_name = CString::new("glCreateProgram").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn() -> GLuint> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glCreateShader() -> fn(shaderType: GLenum) -> GLuint{
    let function_name = CString::new("glCreateShader").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(shaderType: GLenum) -> GLuint> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glShaderSource() -> fn(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar, 
    length: *const GLint
) {
    let function_name = CString::new("glShaderSource").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(shader: GLuint, count: GLsizei,string: *const *const GLchar, length: *const GLint)> (function_pointer)   
}

#[allow(non_snake_case)]
pub unsafe fn _glCompileShader() -> fn(shader: GLuint) {
    let function_name = CString::new("glCompileShader").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(shader: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glAttachShader() -> fn(program: GLuint, shader: GLuint) {
    let function_name = CString::new("glAttachShader").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(program: GLuint, shader: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glLinkProgram() -> fn(program: GLuint) {
    let function_name = CString::new("glLinkProgram").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(program: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glValidateProgram() -> fn(program: GLuint) {
    let function_name = CString::new("glValidateProgram").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(program: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glUseProgram() -> fn(program: GLuint) {
    let function_name = CString::new("glUseProgram").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(program: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glDeleteShader() -> fn(shader: GLuint) {
    let function_name = CString::new("glDeleteShader").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(shader: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glGetShaderiv() -> fn(shader: GLuint, pname: GLenum, params: *const GLint) {
    let function_name = CString::new("glGetShaderiv").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(shader: GLuint, pname: GLenum, params: *const GLint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glGetShaderInfoLog() -> fn(shader: GLuint, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    let function_name = CString::new("glGetShaderInfoLog").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(shader: GLuint, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glDeleteProgram() -> fn(program: GLuint) {
    let function_name = CString::new("glDeleteProgram").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(program: GLuint)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glUniform4f() -> fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    let function_name = CString::new("glUniform4f").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat)> (function_pointer)
}

#[allow(non_snake_case)]
pub unsafe fn _glGetUniformLocation() -> fn(program: GLuint, name: *const GLchar) -> GLint {
    let function_name = CString::new("glGetUniformLocation").unwrap();
    let function_pointer = wglGetProcAddress(function_name.as_ptr());
    transmute::<*const (), fn(program: GLuint, name: *const GLchar) -> GLint> (function_pointer)
}