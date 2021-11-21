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
        #[inline(always)]
        pub fn $fn(&self, $( $param: $typ ),*) {
            (self.$glname)($( $param ),*);
        }
    };
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*) $(-> $ret:ty)*) => {
        #[inline(always)]
        pub fn $fn(&self, $( $param: $typ ),*) $(-> $ret)* {
            (self.$glname)($( $param ),*)
        }
    };
}

macro_rules! map_func_legacy {
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*)) => {
        #[inline(always)]
        pub unsafe fn $fn(&self, $( $param: $typ ),*) {
            $glname($( $param ),*);
        }
    };
    ($fn:ident, $glname:ident: fn($( $param:ident: $typ:ty ),*) $(-> $ret:ty)*) => {
        #[inline(always)]
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
pub type GLintptr = c_int; // not sure if this is correct..
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
pub const GL_TEXTURE0: GLenum = 0x84C0;
pub const GL_TEXTURE1: GLenum = 0x84C1;
pub const GL_TEXTURE2: GLenum = 0x84C2;
pub const GL_TEXTURE3: GLenum = 0x84C3;
pub const GL_TEXTURE4: GLenum = 0x84C4;
pub const GL_TEXTURE5: GLenum = 0x84C5;
pub const GL_TEXTURE6: GLenum = 0x84C6;
pub const GL_TEXTURE7: GLenum = 0x84C7;
pub const GL_TEXTURE8: GLenum = 0x84C8;
pub const GL_TEXTURE9: GLenum = 0x84C9;
pub const GL_TEXTURE10: GLenum = 0x84CA;
pub const GL_TEXTURE11: GLenum = 0x84CB;
pub const GL_TEXTURE12: GLenum = 0x84CC;
pub const GL_TEXTURE13: GLenum = 0x84CD;
pub const GL_TEXTURE14: GLenum = 0x84CE;
pub const GL_TEXTURE15: GLenum = 0x84CF;
pub const GL_TEXTURE16: GLenum = 0x84D0;
pub const GL_TEXTURE17: GLenum = 0x84D1;
pub const GL_TEXTURE18: GLenum = 0x84D2;
pub const GL_TEXTURE19: GLenum = 0x84D3;
pub const GL_TEXTURE20: GLenum = 0x84D4;
pub const GL_TEXTURE21: GLenum = 0x84D5;
pub const GL_TEXTURE22: GLenum = 0x84D6;
pub const GL_TEXTURE23: GLenum = 0x84D7;
pub const GL_TEXTURE24: GLenum = 0x84D8;
pub const GL_TEXTURE25: GLenum = 0x84D9;
pub const GL_TEXTURE26: GLenum = 0x84DA;
pub const GL_TEXTURE27: GLenum = 0x84DB;
pub const GL_TEXTURE28: GLenum = 0x84DC;
pub const GL_TEXTURE29: GLenum = 0x84DD;
pub const GL_TEXTURE30: GLenum = 0x84DE;
pub const GL_TEXTURE31: GLenum = 0x84DF;
pub const GL_SRC_COLOR: GLenum = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const GL_SRC_ALPHA: GLenum = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const GL_DST_ALPHA: GLenum = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const GL_DST_COLOR: GLenum = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const GL_BLEND_DST: GLenum = 0x0BE0;
pub const GL_BLEND_SRC: GLenum = 0x0BE1;
pub const GL_BLEND: GLenum = 0x0BE2;

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
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    pub fn glGenTextures(n: GLsizei, textures: *const GLuint);
    pub fn glBindTexture(target: GLenum, texture: GLuint);
    pub fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, data: *const c_void);
    pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    pub fn glEnable(cap: GLenum);
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}

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
        //glGenTextures: fn(n: GLsizei, textures: *const GLuint),
        //glBindTexture: fn(target: GLenum, texture: GLuint),
        //glTexParameteri: fn(target: GLenum, pname: GLenum, param: GLint),
        //glTexImage2D: fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, data: *const c_void),
        glDeleteTextures: fn(n: GLsizei, textures: *const GLuint),
        glActiveTexture: fn(textures: GLenum),
        glUniform1i: fn(location: GLint, v0: GLint),
        //glBlendFunc: fn(sfactor: GLenum, dfactor: GLenum),
        glUniformMatrix4fv: fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat),
        glEnableVertexArrayAttrib: fn(vaobj: GLuint, index: GLuint),
        glBindTextureUnit: fn(unit: GLuint, texture: GLuint),
        glUniform1iv: fn(location: GLint, count: GLsizei, value: *const GLint),
        glMapBuffer: fn(target: GLenum, access: GLenum),
        glUnmapBuffer: fn(target: GLenum) -> GLboolean,
        glBufferSubData: fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void),
        //glViewport: fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei),
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
    map_func_legacy!{tex_parameter_i, glTexParameteri: fn(target: GLenum, pname: GLenum, param: GLint)}  
    map_func_legacy!{gen_textures, glGenTextures: fn(n: GLsizei, textures: *const GLuint)}  
    map_func_legacy!{bind_texture, glBindTexture: fn(target: GLenum, texture: GLuint)}  
    map_func_legacy!{tex_image_2d, glTexImage2D: fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, data: *const c_void)}  
    map_func_legacy!{delete_textures, glDeleteTextures: fn(n: GLsizei, textures: *const GLuint)}  
    map_func_legacy!{blend_func, glBlendFunc: fn(sfactor: GLenum, dfactor: GLenum)}  
    map_func_legacy!{enable, glEnable: fn(cap: GLenum)}  
    map_func_legacy!{viewport, glViewport: fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)}  
    

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
    //map_func_modern!{gen_textures, glGenTextures: fn(n: GLsizei, textures: *const GLuint)}  
    //map_func_modern!{bind_texture, glBindTexture: fn(target: GLenum, texture: GLuint)}  
    //map_func_modern!{tex_parameter_i, glTexParameteri: fn(target: GLenum, pname: GLenum, param: GLint)}  
    //map_func_modern!{tex_image_2d, glTexImage2D: fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, data: *const c_void)}  
    //map_func_modern!{delete_textures, glDeleteTextures: fn(n: GLsizei, textures: *const GLuint)}  
    map_func_modern!{active_texture, glActiveTexture: fn(textures: GLenum)}  
    map_func_modern!{uniform_1i, glUniform1i: fn(location: GLint, v0: GLint)}  
    //map_func_modern!{blend_func, glBlendFunc: fn(sfactor: GLenum, dfactor: GLenum)}  
    map_func_modern!{uniform_matrix_4fv, glUniformMatrix4fv: fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)}  
    map_func_modern!{enable_vertex_array_attrib, glEnableVertexArrayAttrib: fn(vaobj: GLuint, index: GLuint)}  
    map_func_modern!{bind_texture_unit, glBindTextureUnit: fn(unit: GLuint, texture: GLuint)}  
    map_func_modern!{uniform_1iv, glUniform1iv: fn(location: GLint, count: GLsizei, value: *const GLint)}  
    map_func_modern!{map_buffer, glMapBuffer: fn(target: GLenum, access: GLenum)}  
    map_func_modern!{unmap_buffer, glUnmapBuffer: fn(target: GLenum) -> GLboolean}  
    map_func_modern!{buffer_sub_data, glBufferSubData: fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void)}  
    //map_func_modern!{viewport, glViewport: fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)}  
}