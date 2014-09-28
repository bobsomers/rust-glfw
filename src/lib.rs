extern crate libc;

use libc::{c_int};

mod ffi;
mod platform;

pub fn get_version() -> (i32, i32, i32) {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut rev: c_int = 0;
    unsafe {
        ffi::glfwGetVersion(&mut major, &mut minor, &mut rev);
    }
    return (major as i32, minor as i32, rev as i32);
}
