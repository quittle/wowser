use lazy_static::lazy_static;
use std::{
    io::Write,
    sync::{Mutex, MutexGuard, PoisonError},
};

#[macro_export]
macro_rules! log {
    (DEBUG: $($args:expr),+) => {
        $crate::_internal_log!("DEBUG", None, "", $($args),+);
    };
    (DEBUG[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!("DEBUG", Some($category), "", $($args),+);
    };
    (INFO: $($args:expr),+) => {
        $crate::_internal_log!("INFO", None, "", $($args),+);
    };
    (INFO[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!("INFO", Some($category), "", $($args),+);
    };
    (WARN: $($args:expr),+) => {
        $crate::_internal_log!("WARN", None, "", $($args),+);
    };
    (WARN[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!("WARN", Some($category), "", $($args),+);
    };
    (ERROR: $($args:expr),+) => {
        $crate::_internal_log!("ERROR", None, "", $($args),+);
    };
    (ERROR[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!("ERROR", Some($category), "", $($args),+);
    };
}

#[macro_export]
macro_rules! _internal_log {
    // Recursively format the arguments
    ($level:expr, $category:expr, $prev_arg:expr, $arg:expr, $($args:expr),+) => (
        $crate::_internal_log!($level, $category, format!("{} {:?}", $prev_arg, $arg), $($args),+);
    );

    ($level:expr, $category:expr, $prev_arg:expr, $arg:expr) => (
        $crate::util::logging::_log($level, $category, format!("{} {:?}", $prev_arg, $arg));
    );
}

struct StdOutWriter;
impl Write for StdOutWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if cfg!(test) {
            Ok(buf.len())
        } else {
            std::io::stdout().write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }
}

lazy_static! {
    static ref LOG_OUTPUT: Mutex<Box<dyn Write + Send>> = Mutex::new(Box::new(StdOutWriter));
}

#[allow(unused_must_use)]
pub fn _log(level: &'static str, category: Option<&'static str>, message: String) {
    if let Ok(mut writer) = LOG_OUTPUT.lock() {
        let category_str = category.map(|c| format!("({})", c)).unwrap_or_default();
        // Ignore failure to write
        writer.write_fmt(format_args!("{}{}:{}\n", level, category_str, message));
    }
}

/// Replace the location the logger writes to. Default value is `std::io::stdout()`
pub fn set_logger_output(
    writer: Box<dyn Write + Send>,
) -> Result<(), PoisonError<MutexGuard<'static, Box<(dyn Write + Send + 'static)>>>> {
    let mut output = LOG_OUTPUT.lock()?;
    *output = writer;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        str::from_utf8,
        thread::{self, ThreadId},
    };

    use super::*;

    lazy_static! {
        // A map of thread to buffer to isolate test output
        static ref OUTPUT: Mutex<HashMap<ThreadId, Vec<u8>>> = Mutex::new(HashMap::new());
    }

    struct TestWriter;
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let thread_id = thread::current().id();
            let mut output = OUTPUT.lock().unwrap();
            let thread_local_buffer = output.get_mut(&thread_id).unwrap();
            thread_local_buffer.write_all(buf)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn setup() {
        let thread_id = thread::current().id();

        // Insert or replace the thread-local buffer
        OUTPUT.lock().unwrap().insert(thread_id, vec![]);

        // Make sure to not hold the OUTPUT lock when calling this to prevent a deadlock
        set_logger_output(Box::new(TestWriter)).unwrap();
    }

    fn teardown() {
        set_logger_output(Box::new(StdOutWriter)).unwrap();
    }

    fn get_logged_output() -> String {
        let thread_id = thread::current().id();
        let output = OUTPUT.lock().unwrap();
        let thread_local_buffer = output.get(&thread_id).unwrap();
        from_utf8(thread_local_buffer).unwrap().to_string()
    }

    #[test]
    fn test_tuple_logging_does_not_strip_parentheses() {
        setup();
        log!(INFO: ("tuple", 1), 2, ("tuple", 3));

        assert_eq!(
            get_logged_output(),
            "INFO: (\"tuple\", 1) 2 (\"tuple\", 3)\n"
        );

        teardown();
    }

    #[test]
    fn test_log() {
        setup();

        log!(INFO["net"]: "message", 123);
        log!(INFO: "No category", vec![1, 2, 3]);
        log!(ERROR: "single arg");

        assert_eq!(
            get_logged_output(),
            "INFO(net): \"message\" 123\nINFO: \"No category\" [1, 2, 3]\nERROR: \"single arg\"\n"
        );

        teardown();
    }
}
