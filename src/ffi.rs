#![allow(bad_style)]

use libc::{c_char, c_int};

pub static VERSION_MAJOR: i32 = 3;
pub static VERSION_MINOR: i32 = 0;
pub static VERSION_REVISION: i32 = 4;

extern {
    pub fn glfwGetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    pub fn glfwGetVersionString() -> *const c_char;
}
