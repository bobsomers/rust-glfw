extern crate libc;

use libc::{c_int};
use std::c_str;

pub use ffi::VERSION_MAJOR;
pub use ffi::VERSION_MINOR;
pub use ffi::VERSION_REVISION;

mod ffi;
mod platform;

// TODO: store callbacks
pub struct Glfw;

// TODO: map to FFI errors
pub struct Error;

// TODO: return Result<Glfw, Error>?
// TODO: error on multiple initialization
pub fn init() -> Result<Glfw, Error> {
    match unsafe { ffi::glfwInit() } {
        ffi::FALSE => Err(Error),
        _ => Ok(Glfw)
    }
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

impl Drop for Glfw {
    fn drop(&mut self) {
        unsafe { ffi::glfwTerminate(); }
        println!("Terminated GLFW.");
    }
}
