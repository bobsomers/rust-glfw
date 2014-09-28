// Handles linking to the proper library depending on which platform we're
// compiling for.

#[cfg(target_os = "linux")]
#[link(name = "glfw")]
extern {}

#[cfg(target_os = "macos")]
#[link(name = "glfw3")]
extern {}
