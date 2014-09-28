extern crate glfw;

fn main() {
    let (major, minor, rev) = glfw::version();
    let version_string = glfw::version_string();
    println!("GLFW version is {}.{}.{}", major, minor, rev);
    println!("GLFW version string is {}", version_string);
}
