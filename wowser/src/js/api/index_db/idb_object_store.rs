use super::{IDBIndex, IDBResult};

#[allow(dead_code)]
pub struct IDBObjectStore {
    name: String,
    key_path: Option<String>,
    auto_increment: bool,
    indices: Vec<IDBIndex>,
}

pub enum IDBObjectStoreItem {
    String(String),
}

impl IDBObjectStore {
    pub fn new(name: &str, key_path: Option<&str>, auto_increment: bool) -> Self {
        Self {
            name: name.to_string(),
            key_path: key_path.map(str::to_string),
            auto_increment,
            indices: vec![],
        }
    }

    pub fn add(_item: &IDBObjectStoreItem, _key: Option<&str>) -> IDBResult<()> {
        Ok(())
    }

    pub fn create_index(
        &self,
        index_name: &str,
        key_path: &str,
        unique: Option<bool>,
        multi_entry: Option<bool>,
    ) -> IDBResult<IDBIndex> {
        Ok(IDBIndex::new(
            index_name,
            &self.name,
            Some(key_path),
            multi_entry.unwrap_or(false),
            unique.unwrap_or(false),
        ))
    }
}
