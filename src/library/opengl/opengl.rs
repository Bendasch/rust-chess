#[allow(unused_imports)]
use libc::{c_int, c_uint, c_char, c_uchar, c_float, c_void};
use std::ffi::{CString};
use std::mem::transmute;

/*
    WGL FUNCTION POINTERS
*/
#[link(name = "Opengl32")]
extern "C" {
    pub fn wglGetProcAddress(unnamedParam1: *const c_char) -> *const ();
    pub fn wglGetCurrentContext() -> *const c_char;
}

/* 
    UTILITY TO CLEAN UP FUNCTION BINDING
*/
pub unsafe fn get_function_pointer(name: &str) -> *const () {
    let function_name = CString::new(name).unwrap();
    wglGetProcAddress(function_name.as_ptr())
}

macro_rules! bind {
    (pub struct GL {
        $($name:ident: fn($( $param:ident: $typ:ty ),*) $(-> $ret:ty)*),* $(,)* 
    }) => {
        
        #[allow(non_snake_case)]
        pub struct GL {
            $(pub $name: fn($( $param: $typ ),*) $(-> $ret)*),*
        }

        #[allow(non_snake_case)]
        impl GL {
        
            pub unsafe fn bind() -> GL {
                GL {
                    $(
                        $name: transmute::<*const (), fn($( $param: $typ ),*) $(-> $ret)*> (get_function_pointer(stringify!($name)))
                    ),*
                }
            }
        }
    }
}

macro_rules! map_func_modern {
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*)) => {
        pub fn $fn(&self, $( $param: $typ ),*) {
            (self.$glname)($( $param ),*);
        }
    };
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*) $(-> $ret:ty)*) => {
        pub fn $fn(&self, $( $param: $typ ),*) $(-> $ret)* {
            (self.$glname)($( $param ),*)
        }
    };
}

macro_rules! map_func_legacy {
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*)) => {
        pub unsafe fn $fn(&self, $( $param: $typ ),*) {
            $glname($( $param ),*);
        }
    };
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*) $(-> $ret:ty)*) => {
        pub unsafe fn $fn(&self, $( $param: $typ ),*) $(-> $ret)* {
            $glname($( $param ),*)
        }
    };
}

/*
    OPENGL TYPES
*/
pub type GLenum = c_int;
pub type GLuint = c_uint;
pub type GLint = c_int;
pub type GLfloat = c_float;
pub type GLbitfield = c_int;
pub type GLubyte = c_uchar;
pub type GLsizei = c_int;
pub type GLsizeiptr = c_int; // not c_uchar!!
pub type GLvoid = c_void;
pub type GLboolean = c_char;
pub type GLchar = c_char;

/* 
    OPENGL CONSTANTS
*/
pub const GL_VERSION: GLenum = 0x1F02;
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
pub const GL_TRUE: GLboolean = 1;
pub const GL_FALSE: GLboolean = 0;
pub const GL_NO_ERROR: GLenum = 0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;
pub const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const GL_DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT: GLenum = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT: GLenum = 0x00000400;
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
pub const GL_TEXTURE_1D: GLenum = 0x0DE0;
pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
pub const GL_TEXTURE_WIDTH: GLenum = 0x1000;
pub const GL_TEXTURE_HEIGHT: GLenum = 0x1001;
pub const GL_TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
pub const GL_NEAREST: GLenum = 0x2600;
pub const GL_LINEAR: GLenum = 0x2601;
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const GL_CLAMP_TO_BORDER: GLenum = 0x812D;
pub const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
pub const GL_TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const GL_TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const GL_RED: GLenum = 0x1903;
pub const GL_GREEN: GLenum = 0x1904;
pub const GL_BLUE: GLenum = 0x1905;
pub const GL_ALPHA: GLenum = 0x1906;
pub const GL_RGB: GLenum = 0x1907;
pub const GL_RGBA: GLenum = 0x1908;
pub const GL_RGB8: GLenum = 0x8051;
pub const GL_RGB10: GLenum = 0x8052;
pub const GL_RGB12: GLenum = 0x8053;
pub const GL_RGB16: GLenum = 0x8054;
pub const GL_RGBA2: GLenum = 0x8055;
pub const GL_RGBA4: GLenum = 0x8056;
pub const GL_RGB5_A1: GLenum = 0x8057;
pub const GL_RGBA8: GLenum = 0x8058;
pub const GL_RGB10_A2: GLenum = 0x8059;
pub const GL_RGBA12: GLenum = 0x805A;
pub const GL_RGBA16: GLenum = 0x805B;


/* 
    OPENGL LEGACY FUNCTION POINTERS
    > linked to Opengl32.dll 
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
    pub fn glDrawElements(mode: GLenum, count: GLsizei, _type: GLenum, indices: *const c_void);
    pub fn glGetError() -> GLenum;
}

#[allow(non_snake_case)]
bind!{
    pub struct GL {

        /*
            OPENGL MODERN FUNCTION POINTERS
            > extracted on startup through wglGetProcAddress
        */
        glCreateShader: fn(shaderType: GLenum) -> GLuint,
        glGenBuffers: fn(n: GLsizei, buffers: *mut GLuint),
        glGetUniformLocation: fn(program: GLuint, name: *const GLchar) -> GLint,
        glUniform4f: fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat),
        glDeleteProgram: fn(program: GLuint),
        glGetShaderInfoLog: fn(shader: GLuint, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar),
        glGetShaderiv: fn(shader: GLuint, pname: GLenum, params: *const GLint),
        glDeleteShader: fn(shader: GLuint),
        glUseProgram: fn(program: GLuint),
        glValidateProgram: fn(program: GLuint),
        glLinkProgram: fn(program: GLuint),
        glAttachShader: fn(program: GLuint, shader: GLuint),
        glCompileShader: fn(shader: GLuint),
        glShaderSource: fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint),
        glCreateProgram: fn() -> GLuint,
        glVertexAttribPointer: fn(index: GLuint, size: GLint, _type: GLenum, normalized: GLboolean, stride: GLsizei, function_pointer: *const GLvoid),
        glEnableVertexAttribArray: fn(index: GLuint),
        glBufferData: fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum),
        glBindBuffer: fn(target: GLenum, buffer: GLuint),
        glGetStringi:fn(name: GLenum, index: GLuint) -> *const GLubyte,
        glGenVertexArrays: fn(n: GLsizei, arrays: *const GLuint),
        glBindVertexArray: fn(array: GLuint),
        glDeleteBuffers: fn(n: GLsizei, buffers: *const GLuint),
        glDeleteVertexArrays: fn(n: GLsizei, arrays: *const GLuint),
        glGenTextures: fn(n: GLsizei, textures: *const GLuint),
        glBindTexture: fn(target: GLenum, texture: GLuint),
        glTexParameteri: fn(target: GLenum, pname: GLenum, param: GLint),
        glTexImage2D: fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, data: *const c_void)
    }
}

#[allow(non_snake_case)]
impl GL {
    
    /* 
        Legacy functions
    */
    map_func_legacy!{clear, glClear: fn(mask: GLbitfield)}
    map_func_legacy!{begin, glBegin: fn(mode: GLenum )}
    map_func_legacy!{end, glEnd: fn()}
    map_func_legacy!{vertex_2f, glVertex2f: fn(x: GLfloat, y: GLfloat)} 
    map_func_legacy!{get_integerv, glGetIntegerv: fn(pname: GLenum, data: *mut GLint)}
    map_func_legacy!{get_string, glGetString: fn(name: GLenum) -> *const GLubyte}
    map_func_legacy!{draw_elements, glDrawElements: fn(mode: GLenum, count: GLsizei, _type: GLenum, indices: *const GLvoid)}
    map_func_legacy!{draw_arrays, glDrawArrays: fn(mode: GLenum, first: GLint, count: GLsizei)}
    map_func_legacy!{get_error, glGetError: fn() -> GLenum}  
    
    /*
        Modern functions    
    */
    map_func_modern!{create_shader, glCreateShader: fn(shaderType: GLenum) -> GLuint}
    map_func_modern!{gen_buffers, glGenBuffers: fn(n: GLsizei, buffers: *mut GLuint)}
    map_func_modern!{get_uniform_location, glGetUniformLocation: fn(program: GLuint, name: *const GLchar) -> GLint}
    map_func_modern!{uniform_4f, glUniform4f: fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat)}
    map_func_modern!{delete_program, glDeleteProgram: fn(program: GLuint)}
    map_func_modern!{get_shader_infolog, glGetShaderInfoLog: fn(shader: GLuint, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)}
    map_func_modern!{get_shaderiv, glGetShaderiv: fn(shader: GLuint, pname: GLenum, params: *const GLint)}
    map_func_modern!{delete_shader, glDeleteShader: fn(shader: GLuint)}
    map_func_modern!{use_program, glUseProgram: fn(program: GLuint)}
    map_func_modern!{validate_program, glValidateProgram: fn(program: GLuint)}
    map_func_modern!{link_program, glLinkProgram: fn(program: GLuint)}
    map_func_modern!{attach_shader, glAttachShader: fn(program: GLuint, shader: GLuint)}
    map_func_modern!{compile_shader, glCompileShader: fn(shader: GLuint)}
    map_func_modern!{shader_source, glShaderSource: fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint)}
    map_func_modern!{create_program, glCreateProgram: fn() -> GLuint}
    map_func_modern!{vertex_attrib_pointer, glVertexAttribPointer: fn(index: GLuint, size: GLint, _type: GLenum, normalized: GLboolean, stride: GLsizei, function_pointer: *const GLvoid)}
    map_func_modern!{enable_vertex_attrib_array, glEnableVertexAttribArray: fn(index: GLuint)}
    map_func_modern!{buffer_data, glBufferData: fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum)}
    map_func_modern!{bind_buffer, glBindBuffer: fn(target: GLenum, buffer: GLuint)}
    map_func_modern!{get_stringi, glGetStringi:fn(name: GLenum, index: GLuint) -> *const GLubyte}
    map_func_modern!{gen_vertex_arrays, glGenVertexArrays: fn(n: GLsizei, arrays: *const GLuint)}
    map_func_modern!{bind_vertex_array, glBindVertexArray: fn(array: GLuint)}
    map_func_modern!{delete_buffers, glDeleteBuffers: fn(n: GLsizei, buffers: *const GLuint)}  
    map_func_modern!{delete_vertex_arrays, glDeleteVertexArrays: fn(n: GLsizei, arrays: *const GLuint)}
    map_func_modern!{gen_textures, glGenTextures: fn(n: GLsizei, textures: *const GLuint)}  
    map_func_modern!{bind_texture, glBindTexture: fn(target: GLenum, texture: GLuint)}  
    map_func_modern!{tex_parameter_i, glTexParameteri: fn(target: GLenum, pname: GLenum, param: GLint)}  
    map_func_modern!{tex_image_2d, glTexImage2D: fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, data: *const c_void)}  
}