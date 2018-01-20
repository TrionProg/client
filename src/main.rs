#[macro_use]
extern crate failure;

#[macro_use]
extern crate reactor;

#[macro_use]
extern crate gfx;

extern crate object_pool;
extern crate gfx_window_glutin as gfx_glutin;
extern crate gfx_device_gl as gfx_gl;
extern crate cgmath;
extern crate glutin;
extern crate image;
extern crate zip;

pub mod types;
pub mod consts;

pub mod location;

pub mod resources;
pub mod file_systems;
pub mod storage;
pub mod render;

use storage::{Storage,Resource};
use file_systems::{FileSystem,BasicFS,ReadZipArchive,ReadFileSystem,ReadFile};

fn main() {
    let (render_sender,render_receiver)=reactor::create_channel(types::ThreadSource::Process);
    Storage::initialize(render_sender);
    {
        let mut fs = BasicFS::new(".").unwrap();

        let texture = resources::RgbaTexture::load(&mut fs, "1.png").unwrap();
        let id = texture.insert_to_storage().unwrap();

        println!("{}", id);

        {
            let mut zip = ReadZipArchive::open(&mut fs, "2.zip").unwrap();
            let texture4 = resources::RgbaTexture::load(&mut zip, "1.png").unwrap();
            let id4 = texture4.insert_to_storage().unwrap();

            println!("{}", id4);

            let texture5 = resources::RgbaTexture::load(&mut zip, "1.png").unwrap();
            let id5 = texture5.insert_to_storage().unwrap();

            println!("{}", id5);

            let text=zip.open_file("fdfd.txt").unwrap().read_to_string().unwrap();
            println!("{}", text);
        }

        let texture2 = resources::RgbaTexture::load(&mut fs, "1.png").unwrap();
        let id2 = texture2.insert_to_storage().unwrap();

        println!("{}", id2);

        let texture3 = resources::RgbaTexture::load(&mut fs, "1.png").unwrap();
        let id3 = texture3.insert_to_storage().unwrap();

        println!("{}", id3);
    }
    //println!("{}",get_storage().get_a());
    Storage::delete();
    println!("Hello, world!");
}
