extern crate glfw;

fn main() {
    let (major, minor, rev) = glfw::get_version();
    let version_string = glfw::get_version_string();
    println!("GLFW version is {}.{}.{}", major, minor, rev);
    println!("GLFW version string is {}", version_string);
    println!("GLFW version macro is {}.{}.{}", glfw::VERSION_MAJOR,
                                               glfw::VERSION_MINOR,
                                               glfw::VERSION_REVISION);
    let result = glfw::init();
    let glfw = result.ok().unwrap();
    println!("Initialized GLFW.");

    let result2 = glfw::init();
    match result2 {
        Err(_) => println!("Failed to initialize GLFW twice."),
        Ok(_) => println!("Succeeded initializing GLFW twice.")
    }
}
