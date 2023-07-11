use super::{IDBObjectStore, IDBResult, IndexDbVersion};

#[allow(dead_code)]
pub struct IDBDatabase {
    name: String,
    version: IndexDbVersion,
    object_store_names: Vec<String>,
}

impl IDBDatabase {
    pub fn new(name: &str, version: IndexDbVersion) -> Self {
        Self {
            name: name.to_string(),
            version,
            object_store_names: vec![],
        }
    }

    pub fn create_object_store(
        &mut self,
        name: &str,
        key_path: Option<&str>,
        auto_increment: Option<bool>,
    ) -> IDBResult<IDBObjectStore> {
        Ok(IDBObjectStore::new(
            name,
            key_path,
            auto_increment.unwrap_or(false),
        ))
    }
}
