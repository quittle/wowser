use lazy_static::lazy_static;
use std::{
    cmp::Ordering,
    io::Write,
    sync::{Mutex, MutexGuard, PoisonError},
};

#[macro_export]
macro_rules! log {
    (DEBUG: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Debug, None, "", $($args),+)
    };
    (DEBUG[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Debug, Some($category), "", $($args),+)
    };
    (INFO: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Info, None, "", $($args),+)
    };
    (INFO[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Info, Some($category), "", $($args),+)
    };
    (WARN: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Warn, None, "", $($args),+)
    };
    (WARN[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Warn, Some($category), "", $($args),+)
    };
    (ERROR: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Error, None, "", $($args),+);
    };
    (ERROR[$category:expr]: $($args:expr),+) => {
        $crate::_internal_log!($crate::util::logging::LogLevel::Error, Some($category), "", $($args),+);
    };
}

#[macro_export]
macro_rules! _internal_log {
    // Recursively format the arguments
    ($level:expr, $category:expr, $prev_arg:expr, $arg:expr, $($args:expr),+) => (
        $crate::_internal_log!($level, $category, format!("{} {:?}", $prev_arg, $arg), $($args),+)
    );

    ($level:expr, $category:expr, $prev_arg:expr, $arg:expr) => (
        $crate::util::logging::_log($level, $category, format!("{} {:?}", $prev_arg, $arg))
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
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);
}

#[allow(unused_must_use)]
pub fn _log(level: LogLevel, category: Option<&'static str>, message: String) {
    if let Ok(log_level) = LOG_LEVEL.lock() {
        if level < *log_level {
            return;
        }

        if let Ok(mut writer) = LOG_OUTPUT.lock() {
            let category_str = category.map(|c| format!("({})", c)).unwrap_or_default();
            let level_str: &'static str = level.into();
            // Ignore failure to write
            writer.write_fmt(format_args!("{}{}:{}\n", level_str, category_str, message));
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for &'static str {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Ordering by severity, that is a higher severity level, such as Warn is greater than the severity
/// of Info.
impl From<LogLevel> for u8 {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
        }
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a: u8 = (*self).into();
        let b: u8 = (*other).into();
        a.partial_cmp(&b)
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: u8 = (*self).into();
        let b: u8 = (*other).into();
        a.cmp(&b)
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

pub fn set_logger_level(
    level: LogLevel,
) -> Result<LogLevel, PoisonError<MutexGuard<'static, LogLevel>>> {
    let mut level_guard = LOG_LEVEL.lock()?;
    let prev_level = *level_guard;
    *level_guard = level;
    Ok(prev_level)
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
        set_logger_level(LogLevel::Info).unwrap();
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

    #[test]
    fn test_set_log_level() {
        setup();

        log!(WARN: 123);
        assert_eq!(get_logged_output(), "WARN: 123\n");

        assert_eq!(LogLevel::Info, set_logger_level(LogLevel::Warn).unwrap());
        log!(WARN: 456);
        assert_eq!(get_logged_output(), "WARN: 123\nWARN: 456\n");

        assert_eq!(LogLevel::Warn, set_logger_level(LogLevel::Error).unwrap());
        log!(WARN: 789);
        assert_eq!(get_logged_output(), "WARN: 123\nWARN: 456\n");

        assert_eq!(LogLevel::Error, set_logger_level(LogLevel::Error).unwrap());

        teardown();
    }
}
