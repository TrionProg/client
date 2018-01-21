use std;
use gfx_gl;

//use types::*;

use resources::lods::*;

use render::storage::Storage;
use render::storage::{Resource,ResourcePool};

use failure::Error;


impl Resource<ObjectLod> for ObjectLod {
    fn get_pool(storage:&mut Storage) -> &mut ResourcePool<ObjectLod,Self> {
        &mut storage.object_lods
    }

    fn new(mut resource:ObjectLod, storage:&mut Storage) -> Result<Self,Error> {
        resource.mesh.index(storage)?;
        resource.texture.index(storage)?;

        ok!(resource)
    }
}
