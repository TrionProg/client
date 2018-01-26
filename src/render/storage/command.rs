
pub trait StorageCommandTrait {
    fn process(self, storage:&mut RenderStorage) -> Result<(),Error>;
}

pub enum StorageCommand<R:Resource>{
    Insert(R),
    Delete(ResourceSlot),
}

impl<R:Resource> StorageCommandTrait for StorageCommand<R>{
    fn process(self, storage:&mut RenderStorage) -> Result<(),Error> {
        match self {
            StorageCommand::Insert(resource) => {
                let render_resource=R::RR::new(resource, storage)?;
                render_resource.insert_to_storage(storage)?;
            },
            StorageCommand::Delete(slot) => R::RR::delete_from_storage(slot, storage)?
        }

        ok!()
    }
}

impl<R:Resource> Into<RenderCommand> for StorageCommand<R>{
    fn into(self) -> RenderCommand {
        RenderCommand::StorageCommand(Box::new(self))
    }
}