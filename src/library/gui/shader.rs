use crate::gl;
use crate::library::gui::opengl::*;
use crate::library::gui::utils::*;
use std::{
    ffi::{CString, CStr},
    ptr::{null_mut},
    fs::File,
    io::Read,
    collections::HashMap,
    rc::Rc,
};
use libc::{c_uint, c_char};
use crate::library::gui::gl_maths::Mat4;

pub struct Shader {
    gl: Rc<GL>,
    shader_id: c_uint,
    file_path: String,
    uniforms: HashMap<String, i32> 
}
enum ShaderType {
    None,
    Vertex,
    Fragment
}

impl Shader {

    pub unsafe fn new(file_path: String, gl: Rc<GL>) -> Shader {

        let (vert_shader_str, frag_shader_str) = Shader::parse_file(file_path.as_str());

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

        Shader{ gl, shader_id: program, file_path: file_path, uniforms: HashMap::new() }
    }

    pub fn get_id(&self) -> &c_uint {
        &self.shader_id
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub unsafe fn bind(&self) {
        gl!(self.gl.use_program(self.shader_id));  
    }

    pub unsafe fn unbind(&self) {
        gl!(self.gl.use_program(0));  
    }

    pub unsafe fn compile(gl_type: GLenum, source: CString, gl: &GL) -> GLuint {
        
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
                GL_FRAGMENT_SHADER =>  println!("Fragment shader failed."),
                _ => println!("Other shader failed...")
            };
            println!("Error: {:?}", CStr::from_ptr(msg_pointer).to_str());
            return 0;
        }
        
        return id;
    }

    fn parse_file(file_path: &str) -> (CString, CString) {
        
        //let shader_file_name: &str = "./src/library/opengl/simple.shader";
        let mut file = File::open(file_path).expect(format!("Couldn't read shader file {}", file_path).as_str());
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(format!("Couldn't read contents of file {}", file_path).as_str());
        let lines: Vec<&str> = contents.split("\n").collect(); 
        
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
                ShaderType::Vertex => { vert_shader_str.push_str(line); vert_shader_str.push_str("\n"); },
                ShaderType::Fragment => { frag_shader_str.push_str(line); frag_shader_str.push_str("\n"); },
                ShaderType::None => continue
            }
        }
        
        let vert_shader_str = CString::new(vert_shader_str).unwrap();
        let frag_shader_str = CString::new(frag_shader_str).unwrap();
        
        (vert_shader_str, frag_shader_str)
    }
    
    pub unsafe fn set_uniform_4f(&mut self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        self.cache_uniform_location(name);
        let location = self.get_uniform_location(name);
        gl!(self.gl.uniform_4f(*location, v0, v1, v2, v3));
    }
    
    pub unsafe fn set_uniform_1i(&mut self, name: &str, v0: i32) {
        self.cache_uniform_location(name);
        let location = self.get_uniform_location(name);
        gl!(self.gl.uniform_1i(*location, v0));
    }
    
    pub unsafe fn set_uniform_1iv(&mut self, name: &str, vec: Vec<GLint>) {
        self.cache_uniform_location(name);
        let location = self.get_uniform_location(name);
        gl!(self.gl.uniform_1iv(*location, vec.len() as i32, &vec[0] as *const GLint));
    }
    
    pub unsafe fn set_uniform_mat4f(&mut self, name: &str, mat: Mat4) {
        self.cache_uniform_location(name);
        let location = self.get_uniform_location(name);
        gl!(self.gl.uniform_matrix_4fv(*location, 1, GL_FALSE, &mat.c0[0] as *const GLfloat));
    }

    unsafe fn cache_uniform_location(&mut self, name: &str) {
        if self.uniforms.contains_key(name) {
            return;
        }
        let uniform = CString::new(name).unwrap();
        gl!(let location = self.gl.get_uniform_location(self.shader_id, uniform.as_ptr() as *const GLchar));
        self.uniforms.insert(String::from(name), location);
    }

    unsafe fn get_uniform_location(&self, name: &str) -> &i32 {    
        self.uniforms.get(name).unwrap()
    }

}

impl Drop for Shader { 
    
    fn drop(&mut self) {
        unsafe{ 
            gl!(self.gl.delete_program(self.shader_id));
        }
    }
}

#[cfg(test)]
pub mod tests {
    
    use super::*;
    
    #[test]
    fn read_shaders_from_file_vertex() {
        let (vertex, _) = Shader::parse_file("./src/library/opengl/simple.shader");
        let vertex_string = String::from(vertex.to_str().unwrap());
        let line_vec: Vec<&str> = vertex_string.split("\n").collect();
        assert_eq!(line_vec[0].trim(), "#version 330 core");
    }
    
    #[test]
    fn read_shaders_from_file_fragment() {
        let (_, fragment) = Shader::parse_file("./src/library/opengl/simple.shader");
        let fragment_string = String::from(fragment.to_str().unwrap());
        let line_vec: Vec<&str> = fragment_string.split("\n").collect();
        assert_eq!(line_vec[0].trim(), "#version 330 core");
    }
}