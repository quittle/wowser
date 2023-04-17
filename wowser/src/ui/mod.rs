mod error;
mod ui_event_processor;
mod window;

pub use error::{UiError, UiResult};
pub use ui_event_processor::UiEventProcessor;
pub use window::Window;

#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    pub fn lock_for_ui_threads<F>(f: F)
    where
        F: FnOnce(),
    {
        lazy_static! {
            pub static ref UI_TEST_LOCK: Mutex<()> = Mutex::new(());
        }

        let lock = UI_TEST_LOCK.lock();
        f();
        drop(lock);
    }
}
