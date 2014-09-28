extern crate glfw;

fn main() {
    let (major, minor, rev) = glfw::get_version();
    println!("GLFW version is {}.{}.{}", major, minor, rev);
}
