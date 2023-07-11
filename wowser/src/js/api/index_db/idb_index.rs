use super::{IDBObjectStoreItem, IDBResult, IndexDbCount};

#[allow(dead_code)]
pub struct IDBIndex {
    name: String,
    object_store: String,
    key_path: Option<String>,
    multi_entry: bool,
    unique: bool,
    entries: Vec<IDBObjectStoreItem>,
}

impl IDBIndex {
    pub fn new(
        name: &str,
        object_store: &str,
        key_path: Option<&str>,
        multi_entry: bool,
        unique: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            object_store: object_store.to_string(),
            key_path: key_path.map(str::to_string),
            multi_entry,
            unique,
            entries: vec![],
        }
    }

    pub async fn count(&self, key: Option<&str>) -> IndexDbCount {
        if key.is_none() {
            self.entries.len().try_into().unwrap()
        } else {
            0
        }
    }

    pub async fn get(&self, _key: Option<&str>) -> IDBResult<Option<IDBObjectStoreItem>> {
        Ok(None)
    }
}
