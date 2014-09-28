extern crate libc;

use libc::{c_int};
use std::c_str;

pub use ffi::VERSION_MAJOR;
pub use ffi::VERSION_MINOR;
pub use ffi::VERSION_REVISION;

mod ffi;
mod platform;

pub fn init() -> bool {
    match unsafe { ffi::glfwInit() } {
        ffi::FALSE => false,
        _ => true
    }
}

pub fn terminate() {
    unsafe { ffi::glfwTerminate(); }
}

pub fn get_version() -> (i32, i32, i32) {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut rev: c_int = 0;
    unsafe {
        ffi::glfwGetVersion(&mut major, &mut minor, &mut rev);
    }
    return (major as i32, minor as i32, rev as i32);
}

pub fn get_version_string() -> String {
    let version_cstr = unsafe {
        c_str::CString::new(ffi::glfwGetVersionString(), false)
    };
    version_cstr.as_str().unwrap().to_string()
}
