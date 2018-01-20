use gfx;
use gfx::traits::FactoryExt;

use gfx_gl;
use gfx_gl::Factory;

use resources;

use render::resources::*;

use super::{Resource,ResourcePool};

pub struct Storage {
    //pub gfx_factory: Factory,

    pub rgba_textures:ResourcePool<resources::RgbaTexture, RgbaTexture>,
}

