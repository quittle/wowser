extern crate libc;

mod glfw;

pub use glfw::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        glfw_init();
    }
}
