

pub mod resources;

pub mod commands;
pub use self::commands::{RenderCommand,StorageCommand};

pub mod storage;
pub use self::storage::Storage;

use types::ThreadSource;
use reactor;

pub type RenderSender=reactor::Sender<ThreadSource,RenderCommand>;
/*
pub enum StorageCommand{
    CreateResourceCommand(CreateResourceCommand),
    DeleteResourceCommand(DeleteResourceCommand),
}

impl Into<RenderCommand> for CreateResourceCommand{
    fn into(self) -> RenderCommand {
        RenderCommand::StorageCommand(StorageCommand::CreateResourceCommand(self))
    }
}

impl Into<RenderCommand> for DeleteResourceCommand{
    fn into(self) -> RenderCommand {
        RenderCommand::StorageCommand(StorageCommand::DeleteResourceCommand(self))
    }
}
*/