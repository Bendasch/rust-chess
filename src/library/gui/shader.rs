use crate::gl;
use crate::library::gui::{maths::Matrix4, opengl::*, utils::*};
use libc::{c_char, c_uint};
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    ptr::null_mut,
    rc::Rc,
};

pub struct Shader {
    gl: Rc<GL>,
    shader_id: c_uint,
    file_path: String,
    uniforms: HashMap<String, i32>,
}
enum ShaderType {
    None,
    Vertex,
    Fragment,
}

impl Shader {
    pub fn new(file_path: String, gl: Rc<GL>) -> Shader {
        let (vert_shader_str, frag_shader_str) = Shader::parse_file(file_path.as_str());

        unsafe {
            gl!(let program = gl.create_program());

            if program == 0 {
                panic!("No program could be created");
            }

            let vert_shader_id = Shader::compile(GL_VERTEX_SHADER, vert_shader_str, &gl);
            let frag_shader_id = Shader::compile(GL_FRAGMENT_SHADER, frag_shader_str, &gl);

            gl!(gl.attach_shader(program, vert_shader_id));
            gl!(gl.attach_shader(program, frag_shader_id));

            gl!(gl.link_program(program));
            gl!(gl.validate_program(program));

            gl!(gl.delete_shader(vert_shader_id));
            gl!(gl.delete_shader(frag_shader_id));

            Shader {
                gl,
                shader_id: program,
                file_path,
                uniforms: HashMap::new(),
            }
        }
    }

    pub fn get_id(&self) -> &c_uint {
        &self.shader_id
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn bind(&self) {
        unsafe {
            gl!(self.gl.use_program(self.shader_id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl!(self.gl.use_program(0));
        }
    }

    pub fn compile(gl_type: GLenum, source: CString, gl: &GL) -> GLuint {
        unsafe {
            gl!(let id = gl.create_shader(gl_type));
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
                match gl_type {
                    GL_VERTEX_SHADER => println!("Vertex shader failed."),
                    GL_FRAGMENT_SHADER => println!("Fragment shader failed."),
                    _ => println!("Other shader failed..."),
                };
                println!("Error: {:?}", CStr::from_ptr(msg_pointer).to_str());
                return 0;
            }

            id
        }
    }

    fn parse_file(file_path: &str) -> (CString, CString) {
        let mut file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Couldn't read shader file {}", file_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|_| panic!("Couldn't read contents of file {}", file_path));
        let lines: Vec<&str> = contents.split('\n').collect();

        let (mut vert_shader_str, mut frag_shader_str) = (String::new(), String::new());

        let mut mode = ShaderType::None;
        for line in lines {
            if line.trim().starts_with("#shader") {
                if line.trim() == "#shader vertex" {
                    mode = ShaderType::Vertex;
                } else if line.trim() == "#shader fragment" {
                    mode = ShaderType::Fragment;
                }
                continue;
            }
            match mode {
                ShaderType::Vertex => {
                    vert_shader_str.push_str(line);
                    vert_shader_str.push('\n');
                }
                ShaderType::Fragment => {
                    frag_shader_str.push_str(line);
                    frag_shader_str.push('\n');
                }
                ShaderType::None => continue,
            }
        }

        let vert_shader_str = CString::new(vert_shader_str).unwrap();
        let frag_shader_str = CString::new(frag_shader_str).unwrap();

        (vert_shader_str, frag_shader_str)
    }

    pub fn set_uniform_4f(&mut self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            self.cache_uniform_location(name);
            let location = self.get_uniform_location(name);
            gl!(self.gl.uniform_4f(*location, v0, v1, v2, v3));
        }
    }

    pub fn set_uniform_1i(&mut self, name: &str, v0: i32) {
        unsafe {
            self.cache_uniform_location(name);
            let location = self.get_uniform_location(name);
            gl!(self.gl.uniform_1i(*location, v0));
        }
    }

    pub fn set_uniform_1iv(&mut self, name: &str, vec: Vec<GLint>) {
        unsafe {
            self.cache_uniform_location(name);
            let location = self.get_uniform_location(name);
            gl!(self
                .gl
                .uniform_1iv(*location, vec.len() as i32, &vec[0] as *const GLint));
        }
    }

    pub fn set_uniform_mat4f(&mut self, name: &str, mat: Matrix4<f32>) {
        unsafe {
            self.cache_uniform_location(name);
            let location = self.get_uniform_location(name);
            gl!(self
                .gl
                .uniform_matrix_4fv(*location, 1, GL_FALSE, &mat[(0, 0)] as *const GLfloat));
        }
    }

    fn cache_uniform_location(&mut self, name: &str) {
        if self.uniforms.contains_key(name) {
            return;
        }
        let uniform = CString::new(name).unwrap();
        unsafe {
            gl!(let location = self.gl.get_uniform_location(self.shader_id, uniform.as_ptr() as *const GLchar));
            self.uniforms.insert(String::from(name), location);
        }
    }

    fn get_uniform_location(&self, name: &str) -> &i32 {
        self.uniforms.get(name).unwrap()
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl!(self.gl.delete_program(self.shader_id));
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn read_shaders_from_file_vertex() {
        let (vertex, _) = Shader::parse_file("./src/library/gui/res/simple.shader");
        let vertex_string = String::from(vertex.to_str().unwrap());
        let line_vec: Vec<&str> = vertex_string.split("\n").collect();
        assert_eq!(line_vec[0].trim(), "#version 330 core");
    }

    #[test]
    fn read_shaders_from_file_fragment() {
        let (_, fragment) = Shader::parse_file("./src/library/gui/res/simple.shader");
        let fragment_string = String::from(fragment.to_str().unwrap());
        let line_vec: Vec<&str> = fragment_string.split("\n").collect();
        assert_eq!(line_vec[0].trim(), "#version 330 core");
    }
}
