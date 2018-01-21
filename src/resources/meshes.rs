
use std::fs::File;
use std::io::{Read,Cursor};

use render;

use file_systems::{ReadFileSystem,ReadFile};

use failure::Error;

use storage::{Resource,ResourceID,ResourcePool};
use storage::Storage;

use super::ResourceType;

pub struct P3N3TcMesh {

}

impl Resource for P3N3TcMesh {
    type RR=render::resources::P3N3TcMesh;

    fn get_type() -> ResourceType {
        ResourceType::P3N3TcMesh
    }

    fn get_pool(storage:&Storage) -> &ResourcePool<P3N3TcMesh> {
        &storage.p3n3tc_meshes
    }
}

pub type P3N3TcMeshID = ResourceID<P3N3TcMesh>;