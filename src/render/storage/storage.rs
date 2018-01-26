use gfx;
use gfx::traits::FactoryExt;

use resources;

use render::resources::*;

use failure::Error;

use super::{Resource,ResourcePool};
use super::{StorageCommand,StorageCommandTrait};
use super::Factory;

pub struct Storage {
    pub gfx_factory: Factory,

    pub rgba_textures:ResourcePool<resources::RgbaTexture, RgbaTexture>,

    pub p3n3tc_meshes:ResourcePool<resources::P3N3TcMesh, P3N3TcMesh>,

    pub object_lods:ResourcePool<resources::ObjectLod, resources::ObjectLod>,
}

impl Storage {
    pub fn new(gfx_factory:Factory) -> Self {
        Storage{
            gfx_factory,

            rgba_textures:ResourcePool::new(),

            p3n3tc_meshes:ResourcePool::new(),

            object_lods:ResourcePool::new()
        }
    }

    pub fn process_command(&mut self, command:Box<StorageCommandTrait>) -> Result<(),Error> {
        ok!()
    }
}

