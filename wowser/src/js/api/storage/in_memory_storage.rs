use std::collections::HashMap;

use super::Storage;

#[derive(Default)]
pub struct InMemoryStorage {
    contents: HashMap<String, String>,
}

impl Storage for InMemoryStorage {
    fn length(&self) -> u32 {
        self.contents
            .len()
            .try_into()
            .expect("Unexpected content lengths")
    }

    fn key(&self, index: u32) -> Option<&str> {
        let result = TryInto::<usize>::try_into(index);
        match result {
            Ok(i) => self.contents.keys().nth(i).map(String::as_str),
            Err(_) => None,
        }
    }

    fn get_item(&self, key: &str) -> Option<&str> {
        self.contents.get(key).map(String::as_str)
    }

    fn set_item(&mut self, key: &str, value: &str) {
        self.contents.insert(key.to_string(), value.to_string());
    }

    fn remove_item(&mut self, key: &str) {
        self.contents.remove(key);
    }

    fn clear(&mut self) {
        self.contents.clear();
    }
}
