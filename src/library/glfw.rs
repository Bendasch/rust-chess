use libc::{c_int, c_char};

#[allow(missing_copy_implementations)]
pub enum GLFWmonitor {}

#[allow(missing_copy_implementations)]
pub enum GLFWwindow {}


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
}