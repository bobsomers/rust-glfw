#![allow(bad_style)]

use libc::{c_char, c_int};

pub static FALSE: c_int = 0;

pub static VERSION_MAJOR: i32 = 3;
pub static VERSION_MINOR: i32 = 0;
pub static VERSION_REVISION: i32 = 4;

pub type GLFWerrorfun = extern fn(c_int, *const c_char);

extern {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();

    pub fn glfwGetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    pub fn glfwGetVersionString() -> *const c_char;

    pub fn glfwSetErrorCallback(cbfun: Option<GLFWerrorfun>) -> Option<GLFWerrorfun>;
}
