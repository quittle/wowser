use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::prelude::FileExt;
use std::path::Path;

use super::{InMemoryStorage, Storage, StorageError};

pub struct FileBasedStorage {
    file: File,
    memory: InMemoryStorage,
}

impl FileBasedStorage {
    pub fn new(path: &Path) -> Result<Self, StorageError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut memory = InMemoryStorage::default();
        parse_file(&mut file, &mut memory)?;

        Ok(FileBasedStorage { file, memory })
    }

    pub fn write_to_disk(&mut self) -> Result<(), StorageError> {
        self.file.set_len(0)?;
        for index in 0..self.memory.length() {
            let key = self
                .memory
                .key(index)
                .expect("Invariant of length of items");
            let key_bytes = key.as_bytes();
            let key_len_bytes = key_bytes.len().to_be_bytes();
            self.file.write_all(&key_len_bytes)?;
            self.file.write_all(key_bytes)?;

            let value = self.memory.get_item(key).expect("Key just extracted");
            let value_bytes = value.as_bytes();
            let value_len_bytes = value_bytes.len().to_be_bytes();
            self.file.write_all(&value_len_bytes)?;
            self.file.write_all(value_bytes)?;
        }
        self.file.flush()?;
        Ok(())
    }
}

impl Storage for FileBasedStorage {
    fn length(&self) -> u32 {
        self.memory.length()
    }

    fn key(&self, index: u32) -> Option<&str> {
        self.memory.key(index)
    }

    fn get_item(&self, key: &str) -> Option<&str> {
        self.memory.get_item(key)
    }

    fn set_item(&mut self, key: &str, value: &str) {
        self.memory.set_item(key, value)
    }

    fn remove_item(&mut self, key: &str) {
        self.memory.remove_item(key)
    }

    fn clear(&mut self) {
        self.memory.clear()
    }
}

impl Drop for FileBasedStorage {
    fn drop(&mut self) {
        if let Err(err) = self.write_to_disk() {
            log!(ERROR: "Unable to save storage FileBasedStorage:", err);
        }
    }
}

fn parse_file(file: &mut File, storage: &mut InMemoryStorage) -> Result<(), StorageError> {
    let file_len: u64 = file.metadata()?.len();
    let mut length_bytes = [0u8; 8];
    let mut offset: u64 = 0;
    while offset < file_len {
        file.read_exact_at(&mut length_bytes, offset)?;
        let key_len = u64::from_be_bytes(length_bytes);
        offset += length_bytes.len() as u64;
        let mut key_bytes = vec![0u8; key_len as usize];
        file.read_exact_at(&mut key_bytes, offset)?;
        offset += key_len;

        file.read_exact_at(&mut length_bytes, offset)?;
        let value_len = u64::from_be_bytes(length_bytes);
        offset += length_bytes.len() as u64;
        let mut value_bytes = vec![0u8; value_len as usize];
        file.read_exact_at(&mut value_bytes, offset)?;
        offset += value_len;

        storage.set_item(
            std::str::from_utf8(&key_bytes)?,
            std::str::from_utf8(&value_bytes)?,
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use crate::{
        function_name,
        js::api::storage::{Storage, StorageError},
    };

    use super::FileBasedStorage;

    fn test_file(file_name: &str) -> PathBuf {
        let path_buf = env::temp_dir().join(file_name);
        if path_buf.try_exists().expect("Failed to look up {path_buf}") {
            fs::remove_file(&path_buf).expect("Failed to remove file")
        }
        path_buf
    }

    #[test]
    fn test_serialization() -> Result<(), StorageError> {
        let file = test_file(function_name!());
        assert!(!file.exists());
        {
            let _ = FileBasedStorage::new(&file)?;
        }
        assert_eq!(0, file.metadata()?.len());

        {
            let mut storage = FileBasedStorage::new(&file)?;
            assert_eq!(0, storage.length());
            storage.set_item("abc", "123");
            storage.set_item("def", "456");
            storage.set_item("abc", "789");
        }
        assert!(file.metadata()?.len() > 10);

        {
            let storage = FileBasedStorage::new(&file)?;
            assert_eq!(2, storage.length());
            assert_eq!(Some("789"), storage.get_item("abc"));
            assert_eq!(Some("456"), storage.get_item("def"));
        }

        Ok(())
    }
}
