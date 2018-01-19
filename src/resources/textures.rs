
use std::fs::File;
use std::io::{Read,Cursor};

use image;
use image::{GrayImage, GrayAlphaImage, RgbImage, RgbaImage};

use failure::Error;

use storage::Resource;
use storage::ResourceID;
use storage::Storage;
use storage::ResourcePool;
use storage::{CreateResourceCommand,DeleteResourceCommand};

pub struct RgbaTexture {
    image_buffer:RgbaImage
}

impl RgbaTexture {
    pub fn load(file_name:&str) -> Result<Self,Error> {
        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(err) => bail!(err)//bail!(err, file_name.to_string()),
        };

        let mut buf=Vec::with_capacity(1024*16);

        let cursor=match file.read_to_end(&mut buf) {
            Ok(_) => Cursor::new(buf),
            Err(err) => bail!(err)//return err!(err, file_name.to_string()),
        };

        let image_buffer = image::load(cursor, image::PNG).unwrap().to_rgba();

        let texture=RgbaTexture {
            image_buffer
        };

        ok!(texture)
    }
}

impl Resource for RgbaTexture {
    type RR=render::resources::RgbaTexture;

    fn print_type() -> &'static str {
        "Rgba Texture"
    }

    fn get_pool(storage:&Storage) -> &ResourcePool<RgbaTexture> {
        &storage.rgba_textures
    }

    fn create_resource_command(self) -> CreateResourceCommand {
        CreateResourceCommand::RgbaTexture(self)
    }

    fn delete_resource_command(resource_id:ResourceID<Self>) -> DeleteResourceCommand {
        DeleteResourceCommand::RgbaTexture(resource_id)
    }
}

pub type RgbaTextureID = ResourceID<RgbaTexture>;