use std::env;

/// Parses an environment variable a boolean flag, looking for
pub fn get_bool_env(arg: &str, default: bool) -> bool {
    if let Ok(str_val) = env::var(arg) {
        // Ensure a valid value for the argument, otherwise returning the default.
        if str_val == "1" || str_val.eq_ignore_ascii_case("true") {
            return true;
        } else if str_val == "0" || str_val.eq_ignore_ascii_case("false") {
            return false;
        }
    }
    default
}
