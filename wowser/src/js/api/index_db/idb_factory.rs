use super::{IDBDatabase, IndexDbVersion};

pub struct IDBFactory {}

#[allow(dead_code)]
impl IDBFactory {
    async fn close(&self) {}
    async fn open(name: &str, version: Option<IndexDbVersion>) -> Result<IDBDatabase, String> {
        Ok(IDBDatabase::new(name, version.unwrap_or(1)))
    }
}
