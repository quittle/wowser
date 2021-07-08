mod glfw;

pub use glfw::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        unsafe {
            glfwInit();
            glfwTerminate();
        }
    }
}
