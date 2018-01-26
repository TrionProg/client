
pub mod error;
pub use self::error::StorageError;

pub mod resource;
pub use self::resource::{Resource,ResourcePool};

pub mod storage;
pub use self::storage::Storage;

pub mod command;
use self::command::{StorageCommand,StorageCommandTrait};

pub use gfx_gl::Factory;