use crate::library::glfw::*;
use std::ffi::CString;
use std::ptr::null_mut;

pub fn run() {

    unsafe {

        let window: *mut GLFWwindow;
        let monitor: *mut GLFWmonitor = null_mut();
        let share: *mut GLFWwindow = null_mut();
        
        /* Initialize the library */
        if glfwInit() == 0 {
            return;
        }
        
        /* Create a windowed mode window and its OpenGL context */
        let title = CString::new("Rust chess (OpenGL)").unwrap();

        window = glfwCreateWindow(640, 480, title.as_ptr(), monitor, share);
        if window.is_null() {
            glfwTerminate();
            return;
        }
        
        /* Make the window's context current */
        glfwMakeContextCurrent(window);
        
        /* Loop until the user closes the window */
        while glfwWindowShouldClose(window) == 0 {

            /* Render here */
            //glClear(GL_COLOR_BUFFER_BIT);
            
            /* Swap front and back buffers */
            //glfwSwapBuffers(window);
            
            /* Poll for and process events */
            glfwPollEvents();
        }
        
        glfwTerminate();
    }
}