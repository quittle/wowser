mod event_handlers;
mod idb_database;
mod idb_factory;
mod idb_index;
mod idb_key;
mod idb_key_range;
mod idb_object_store;

pub use event_handlers::*;
pub use idb_database::*;
pub use idb_factory::*;
pub use idb_index::*;
pub use idb_key::*;
pub use idb_key_range::*;
pub use idb_object_store::*;

pub type IndexDbVersion = u64;
pub type IndexDbCount = u64;

pub enum IDBError {
    InvalidState,
    TransactionInactiveError,
    ConstraintError,
    InvalidAccessError,
}

pub type IDBResult<T> = Result<T, IDBError>;

pub trait IndexDb {}
