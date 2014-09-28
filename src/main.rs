extern crate glfw;

fn main() {
    let (major, minor, rev) = glfw::get_version();
    let version_string = glfw::get_version_string();
    println!("GLFW version is {}.{}.{}", major, minor, rev);
    println!("GLFW version string is {}", version_string);
}
