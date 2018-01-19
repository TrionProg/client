
mod error;
pub use self::error::StorageError;

pub mod storage;
pub use self::storage::Storage;

mod global;
pub use self::global::get_storage;

pub mod resource;
pub use self::resource::{Resource,ResourceID,ResourceSlot,ResourcePool};