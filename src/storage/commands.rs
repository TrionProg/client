
use render::RenderCommand;

use resources::textures::*;
/*
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
*/

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

pub enum CreateResourceCommand {
    RgbaTexture(RgbaTexture)
}

pub enum DeleteResourceCommand {
    RgbaTexture(RgbaTextureID)
}

impl From<RgbaTexture> for CreateResourceCommand {
    fn from(resource:RgbaTexture) -> CreateResourceCommand{
        CreateResourceCommand::RgbaTexture(resource)
    }
}

impl From<RgbaTextureID> for DeleteResourceCommand {
    fn from(resource_id:RgbaTextureID) -> DeleteResourceCommand{
        DeleteResourceCommand::RgbaTexture(resource_id)
    }
}