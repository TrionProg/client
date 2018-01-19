

pub mod resources;

use types::ThreadSource;
use reactor;

use storage::StorageCommand;

pub type RenderSender=reactor::Sender<ThreadSource,RenderCommand>;

pub enum RenderCommand{
    StorageCommand(StorageCommand),
}