extern crate image;
use crate::gl;
use crate::library::gui::{
    opengl::*,
    utils::{gl_clear_errors, gl_print_errors},
};
use image::*;
use libc::c_void;
use std::rc::Rc;

pub struct Texture {
    gl: Rc<GL>,
    texture_id: GLuint,
    file_path: String,
    img: RgbaImage,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(file_path: &str, gl: Rc<GL>) -> Texture {
        let img = image::open(file_path).unwrap().flipv().into_rgba8();
        let dim = img.dimensions();
        let raw: &Vec<u8> = img.as_raw();

        let mut texture_id: GLuint = 0;

        unsafe {
            gl!(gl.gen_textures(1, &mut texture_id));
            gl!(gl.bind_texture(GL_TEXTURE_2D, texture_id));
            gl!(gl.tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR));
            gl!(gl.tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR));
            gl!(gl.tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE));
            gl!(gl.tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE));

            gl!(gl.tex_image_2d(
                GL_TEXTURE_2D,
                0,
                GL_RGBA8,
                dim.0 as GLint,
                dim.1 as GLint,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                raw.as_ptr() as *const c_void
            ));
        }

        Texture {
            gl,
            texture_id,
            file_path: String::from(file_path),
            img,
            width: dim.0,
            height: dim.1,
        }
    }

    pub fn get_texture_id(&self) -> GLuint {
        self.texture_id
    }

    pub fn get_width(&self) -> &u32 {
        &self.width
    }

    pub fn get_height(&self) -> &u32 {
        &self.height
    }

    pub fn get_img(&self) -> &RgbaImage {
        &self.img
    }

    pub fn get_path(&self) -> &str {
        &self.file_path
    }

    pub fn bind(&self, slot: usize) {
        unsafe {
            gl!(self.gl.active_texture(GL_TEXTURE0 + (slot as i32)));
            gl!(self.gl.bind_texture(GL_TEXTURE_2D, self.texture_id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl!(self.gl.bind_texture(GL_TEXTURE_2D, 0));
        }
    }

    pub fn bind_texture_unit(&self, unit: u32) {
        unsafe {
            gl!(self.gl.bind_texture_unit(unit, self.texture_id));
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl!(self
                .gl
                .delete_textures(1, &self.texture_id as *const GLuint));
        };
    }
}
