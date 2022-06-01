#[cfg(test)]
#[track_caller]
fn assert_panics<F>(execution: F, message: &str) -> Box<dyn std::any::Any + Send>
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(execution);

    match result {
        Ok(_) => panic!("{message}"),
        Err(err) => err,
    }
}

#[cfg(test)]
mod tests {
    use super::assert_panics;

    #[test]
    fn test_assert_panics() {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| ()));
        let error = assert_panics(
            || panic!("Inner error message"),
            "This error should not be shown",
        );
        std::panic::set_hook(prev_hook);
        let message = error
            .downcast_ref::<&'static str>()
            .expect("Panic error should have been a &str");
        assert_eq!("Inner error message", *message);
    }

    #[test]
    #[should_panic = "Nothing panicked when asserting :/"]
    fn test_assert_panics_no_panic() {
        assert_panics(|| (), "Nothing panicked when asserting :/");
    }
}
