
use std::fs::File;
use std::io::{Read,Cursor};

use render;

use image;
use image::{GrayImage, GrayAlphaImage, RgbImage, RgbaImage};

use file_systems::{ReadFileSystem,ReadFile};

use failure::Error;

use storage::Resource;
use storage::ResourceID;
use storage::Storage;
use storage::ResourcePool;

use super::ResourceType;

pub struct RgbaTexture {
    image_buffer:RgbaImage
}


impl RgbaTexture {
    pub fn load<'a, FS:ReadFileSystem<'a>>(fs:&'a mut FS, file_name:&str) -> Result<Self,Error> {
        let mut file=fs.open_file(file_name)?;
        let cursor=Cursor::new(file.read_to_end()?);

        /*
        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(err) => bail!(err)//bail!(err, file_name.to_string()),//TODO
        };

        let mut buf=Vec::with_capacity(1024*16);

        let cursor=match file.read_to_end(&mut buf) {
            Ok(_) => Cursor::new(buf),
            Err(err) => bail!(err)//return err!(err, file_name.to_string()),
        };
        */

        let image_buffer = image::load(cursor, image::PNG).unwrap().to_rgba();

        let texture=RgbaTexture {
            image_buffer
        };

        ok!(texture)
    }
}

impl Resource for RgbaTexture {
    type RR=render::resources::RgbaTexture;

    fn get_type() -> ResourceType {
        ResourceType::RgbaTexture
    }

    fn get_pool(storage:&Storage) -> &ResourcePool<RgbaTexture> {
        &storage.rgba_textures
    }
}

pub type RgbaTextureID = ResourceID<RgbaTexture>;