use libc::{c_int, c_char, c_void};

#[allow(missing_copy_implementations)]
pub enum GLFWmonitor {}

#[allow(missing_copy_implementations)]
pub enum GLFWwindow {}

pub const GLFW_MOUSE_BUTTON_1: c_int = 0;
pub const GLFW_MOUSE_BUTTON_2: c_int = 1;
pub const GLFW_MOUSE_BUTTON_LEFT: c_int = 0;
pub const GLFW_MOUSE_BUTTON_RIGHT: c_int = 1;
pub const GLFW_RELEASE: c_int = 0;
pub const GLFW_PRESS: c_int = 1;
pub const GLFW_REPEAT: c_int = 2;


#[link(name="glfw3", kind="static")]
#[link(name="user32")]
#[link(name="Gdi32")]
#[link(name="Shell32")]
extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwPollEvents();
    pub fn glfwTerminate();
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwSetMouseButtonCallback(window: *const GLFWwindow, callback: extern fn(*const GLFWwindow, c_int, c_int, c_int)); 
    pub fn glfwGetCursorPos(window: *const GLFWwindow, xpos: *mut f64, ypos: *mut f64);
    pub fn glfwSetWindowUserPointer(window: *const GLFWwindow, pointer: *const c_void);	
    pub fn glfwGetWindowUserPointer(window: *const GLFWwindow) -> *const c_void;
}