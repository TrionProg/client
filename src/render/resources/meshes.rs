use std;
use gfx_gl;

//use types::*;

use resources;

use render::storage::Storage;
use render::storage::{Resource,ResourcePool};

use failure::Error;

use gfx;
use gfx::Factory;

pub struct P3N3TcMesh {
    a:i32,
}

impl Resource<resources::P3N3TcMesh> for P3N3TcMesh {
    fn get_pool(storage:&mut Storage) -> &mut ResourcePool<resources::P3N3TcMesh,Self> {
        &mut storage.p3n3tc_meshes
    }

    fn new(resource:resources::P3N3TcMesh, storage:&mut Storage) -> Result<Self,Error> {
        ok!(P3N3TcMesh{a:4})
    }
}