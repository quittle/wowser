use lazy_static::lazy_static;
use std::{
    io::Write,
    sync::{Mutex, MutexGuard, PoisonError},
};

#[macro_export]
macro_rules! log {
    (INFO: $($arg:tt)+) => (
        $crate::util::logging::log("INFO", None, format!("{:?}", ($($arg)+)));
    );
    (INFO[$category:expr]: $($arg:tt)+) =>(
        $crate::util::logging::log("INFO", Some($category), format!("{:?}", ($($arg)+)));
    );
    (ERROR: $($arg:tt)+) => (
        $crate::util::logging::log("ERROR", None, format!("{:?}", ($($arg)+)));
    );
    (ERROR[$category:expr]: $($arg:tt)+) =>(
        $crate::util::logging::log("ERROR", Some($category), format!("{:?}", ($($arg)+)));
    );
}

lazy_static! {
    static ref LOG_OUTPUT: Mutex<Box<dyn Write + Send>> = Mutex::new(Box::new(std::io::stdout()));
}

#[allow(unused_must_use)]
pub fn log(level: &'static str, category: Option<&'static str>, message: String) {
    if let Ok(mut writer) = LOG_OUTPUT.lock() {
        let cleaned_message = message
            .strip_prefix('(')
            .unwrap_or(&message)
            .strip_suffix(')')
            .unwrap_or(&message);
        let category_str = category.map(|c| format!("({})", c)).unwrap_or_default();
        // Ignore failure to write
        writer.write_fmt(format_args!(
            "{}{}: {}\n",
            level, category_str, cleaned_message
        ));
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
            "INFO: (\"tuple\", 1), 2, (\"tuple\", 3)\n"
        );
    }

    #[test]
    fn test_log() {
        setup();

        log!(INFO["net"]: "message", 123);
        log!(INFO: "No category", vec![1, 2, 3]);
        log!(ERROR: "single arg");

        assert_eq!(
            get_logged_output(),
            "INFO(net): \"message\", 123\nINFO: \"No category\", [1, 2, 3]\nERROR: \"single arg\"\n"
        );
    }
}
