
pub mod error;
use self::error::RenderError;

pub mod render;
pub use self::render::{Render, RenderSender};
pub use self::render::Encoder;

pub mod resources;

pub mod commands;
pub use self::commands::{RenderCommand,StorageCommand};

pub mod storage;
pub use self::storage::Storage;
pub use self::storage::command::StorageCommand;

pub mod window;
pub use self::window::Window;

pub mod targets;
pub use self::targets::Targets;
pub use self::targets::{FinalColorTarget, FinalDepthTarget};

pub mod camera;
pub use self::camera::Camera;

pub mod slots;
pub use self::slots::Slots;

pub mod pipelines;


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