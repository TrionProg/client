
use failure::Error;

use render::RenderSender;

use resources::*;

use super::StorageError;
use super::ResourcePool;


pub struct Storage{
    pub rgba_textures:ResourcePool<RgbaTexture>,
}

impl Storage {
    pub fn initialize(render_sender:RenderSender) {
        use super::global::set_storage;

        let storage=Box::new(Storage::new(render_sender));
        set_storage(storage);
    }

    fn new(render_sender:RenderSender) -> Self{
        Storage{
            rgba_textures:ResourcePool::new(render_sender.clone()),
        }
    }

    pub fn delete(){
        use super::global::delete_storage;
        delete_storage();
    }
}