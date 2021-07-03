/// Gets the fully qualified name of the function this macro is invoked in. This includes all the
/// module names separated by "::".
#[macro_export]
macro_rules! fully_qualified_function_name {
    () => {{
        // Fixed function name of known length
        fn f() {}

        // Grabs the fully qualified name of the function
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Strips off the trailing "::f"
        &name[..name.len() - 3]
    }};
}

/// Gets the name of the function this macro is invoked in.
#[macro_export]
macro_rules! function_name {
    () => {{
        let name = crate::fully_qualified_function_name!();
        &name[name.rfind(':').unwrap() + 1..]
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fully_qualified_function_name() {
        assert_eq!(
            "wowser::util::reflection::test::test_fully_qualified_function_name",
            fully_qualified_function_name!()
        );
    }

    #[test]
    fn test_function_name() {
        assert_eq!("test_function_name", function_name!());
    }
}
