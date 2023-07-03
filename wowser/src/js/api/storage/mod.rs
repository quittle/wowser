mod file_based_storage;
mod in_memory_storage;
mod storage_error;

pub use file_based_storage::*;
pub use in_memory_storage::*;
pub use storage_error::*;

/// <https://developer.mozilla.org/en-US/docs/Web/API/Storage>
/// This should match the JS API in behavior
pub trait Storage {
    fn length(&self) -> u32;
    fn key(&self, index: u32) -> Option<&str>;
    fn get_item(&self, key: &str) -> Option<&str>;
    fn set_item(&mut self, key: &str, value: &str);
    fn remove_item(&mut self, key: &str);
    fn clear(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! storage_test {
        ($name:ident, $body:tt) => {
            mod $name {
                use super::*;

                #[test]
                fn test_in_memory() {
                    fn new_storage() -> Box<dyn Storage> {
                        Box::<InMemoryStorage>::default()
                    }

                    $body
                }
            }
        };
    }

    storage_test!(default, {
        let storage = new_storage();
        assert_eq!(storage.length(), 0);
        assert_eq!(storage.key(0), None);
        assert_eq!(storage.get_item(""), None);
    });

    storage_test!(crud, {
        let mut storage = new_storage();
        storage.set_item("abc", "def");
        storage.set_item("123", "456");
        assert_eq!(storage.length(), 2);

        assert_in!(storage.key(0).unwrap(), ["abc", "123"]);
        assert_in!(storage.key(1).unwrap(), ["abc", "123"]);
        assert_ne!(storage.key(0), storage.key(1));
        assert_eq!(storage.key(2), None);
        assert_eq!(storage.length(), 2);
        assert_eq!(storage.get_item("123"), Some("456"));
        storage.remove_item("123");
        assert_eq!(storage.get_item("123"), None);
        assert_eq!(storage.length(), 1);
        storage.clear();
        assert_eq!(storage.get_item("abc"), None);
        assert_eq!(storage.length(), 0);
    });

    storage_test!(replace, {
        let mut storage = new_storage();
        storage.set_item("abc", "123");
        storage.set_item("abc", "456");
        assert_eq!(storage.get_item("abc"), Some("456"));
    });
}
