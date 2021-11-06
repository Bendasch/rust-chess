#[allow(unused_imports)]
use libc::{c_int, c_uint, c_char, c_uchar, c_float, c_void};

/*
    TYPES
*/
pub type GLenum = c_int;
pub type GLuint = c_uint;
pub type GLint = c_int;
pub type GLfloat = c_float;
pub type GLbitfield = c_int;
pub type GLubyte = c_uchar;
pub type GLsizei = c_int;
pub type GLsizeiptr = c_uchar;
pub type GLvoid = c_void;
pub type GLboolean = c_char;
pub type GLchar = c_char;

/* 
    CONSTANTS
*/
pub const GL_VERSION: GLenum = 0x1F02;
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;

pub const GL_TRUE: GLboolean = 1;
pub const GL_FALSE: GLboolean = 0;

pub const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;     // Indicates the buffers currently enabled for color writing.
pub const GL_DEPTH_BUFFER_BIT: GLenum = 0x00000100;     // Indicates the depth buffer.
pub const GL_ACCUM_BUFFER_BIT: GLenum = 0x00000200;     // Indicates the accumulation buffer.
pub const GL_STENCIL_BUFFER_BIT: GLenum = 0x00000400;   // Indicates the stencil buffer.

pub const GL_POINTS: GLenum = 0x0000; 
pub const GL_LINES: GLenum = 0x0001; 
pub const GL_LINE_LOOP: GLenum = 0x0002; 
pub const GL_LINE_STRIP: GLenum = 0x0003; 
pub const GL_TRIANGLES: GLenum = 0x0004; 
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005; 
pub const GL_TRIANGLE_FAN: GLenum = 0x0006; 
pub const GL_QUADS: GLenum = 0x0007; 
pub const GL_QUAD_STRIP: GLenum = 0x0008; 
pub const GL_POLYGON: GLenum = 0x0009; 

pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_FLOAT: GLenum = 0x1406;

/* 
    LEGACY FUNCTIONS
*/
#[link(name = "Opengl32")]
extern "C" {
    pub fn glClear(mask: GLbitfield);
    pub fn glBegin(mode: GLenum );
    pub fn glEnd();
    pub fn glVertex2f(x: GLfloat, y: GLfloat); 
    pub fn glGetIntegerv(pname: GLenum, data: *mut GLint);
    pub fn glGetString(name: GLenum) -> *const GLubyte;
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
}