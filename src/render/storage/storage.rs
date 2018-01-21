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

    pub p3n3tc_meshes:ResourcePool<resources::P3N3TcMesh, P3N3TcMesh>,

    pub object_lods:ResourcePool<resources::ObjectLod, resources::ObjectLod>,
}

