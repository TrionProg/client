use std;
use gfx_gl;

//use types::*;

use resources;

use render::storage::Storage;
use render::storage::{Resource,ResourcePool};

use failure::Error;

use gfx;
use gfx::Factory;

use gfx::texture::Size;
use gfx::texture::Kind;
use gfx::texture::AaMode;

pub struct RgbaTexture {
    texture:gfx::handle::Texture<gfx_gl::Resources, gfx::format::R8_G8_B8_A8>,
    pub view:gfx::handle::ShaderResourceView<gfx_gl::Resources, [f32; 4]>
}

impl Resource<resources::RgbaTexture> for RgbaTexture {
    fn get_pool(storage:&mut Storage) -> &mut ResourcePool<resources::RgbaTexture,Self> {
        &mut storage.rgba_textures
    }

    fn new(resource:resources::RgbaTexture, storage:&mut Storage) -> Result<Self,Error> {
        let width=image_buffer.width() as Size;
        let height=image_buffer.height() as Size;

        let data=image_buffer.into_vec();

        let (texture, view) = try!( storage.gfx_factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
            Kind::D2(width, height, AaMode::Single),
            &[&data[..]]
        ), Error::CreateTextureError);

        let texture=RgbaTexture {
            texture,
            view
        };

        ok!(texture)
    }
}