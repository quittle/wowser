mod gl;

pub use gl::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        unsafe {
            glFlush();
        }
    }
}
