
use storage::{Resource,ResourceSlot};
use resources::ResourceType;

use failure::Error;

pub enum RenderCommand{
    StorageCommand(Box<StorageCommandTrait + Send>),
}

pub trait StorageCommandTrait {
    fn process(self) -> Result<(),Error>;
}

pub enum StorageCommand<R:Resource>{
    Insert(R),
    Delete(ResourceSlot),
}

impl<R:Resource> StorageCommandTrait for StorageCommand<R>{
    fn process(self) -> Result<(),Error> {
        /*
        match self {
            StorageCommand::Insert(resource) => R::create(resource),
            StorageCommand::Delete(slot) => R::delete(slot)
        }
        */

        ok!()
    }
}

impl<R:Resource> Into<RenderCommand> for StorageCommand<R>{
    fn into(self) -> RenderCommand {
        RenderCommand::StorageCommand(Box::new(self))
    }
}