
use std::fs::File;
use std::io::{Read,Cursor};

use render;

use failure::Error;

use storage::{Resource,ResourceID,ResourcePool};
use storage::Storage;

use super::ResourceType;
use super::textures::*;
use super::meshes::*;

pub struct ObjectLod {
    pub mesh:P3N3TcMeshID,
    pub texture:RgbaTextureID
}

impl ObjectLod {

}

impl Resource for ObjectLod {
    type RR=Self;

    fn get_type() -> ResourceType {
        ResourceType::ObjectLod
    }

    fn get_pool(storage:&Storage) -> &ResourcePool<ObjectLod> {
        &storage.object_lods
    }
}

pub type ObjectLodID = ResourceID<ObjectLod>;
