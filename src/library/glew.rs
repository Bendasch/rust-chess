use libc::{c_int, c_uint, c_char, c_uchar};

#[link(name = "glew32s", kind="static")]
extern "C" {
    pub fn glewInit() -> c_int;
    //pub fn glGetString (name: c_int) -> *const c_char;
}