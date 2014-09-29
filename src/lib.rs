extern crate libc;
extern crate sync;

use libc::{c_int};

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
pub fn init() -> Result<Glfw, Error> {
    use sync::one::{Once, ONCE_INIT};

    static mut INIT: Once = ONCE_INIT;
    let mut result = Err(Error);
    unsafe {
        INIT.doit(|| {
            if ffi::glfwInit() != ffi::FALSE {
                result = Ok(Glfw)
            }
        });
    }

    result
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
    use std::string::raw;
    use std::mem;

    unsafe { raw::from_buf(mem::transmute(ffi::glfwGetVersionString())) }
}

impl Drop for Glfw {
    fn drop(&mut self) {
        unsafe { ffi::glfwTerminate(); }
        println!("Terminated GLFW.");
    }
}
