
use types::{RgbaTextureID,TerrainMeshID,ObjectMeshID};

use object_pool::growable::ID;
use storage::{TextureID,MeshID};

use render::SetSlot;

pub struct Slots {
}

impl Slots {
    pub fn new() -> Self {
        /*
        let terrain_textures=vec![RgbaTextureID::zeroed();5];
        let wall_meshes=vec![TerrainMeshID::zeroed();16];
        let hole_meshes=vec![TerrainMeshID::zeroed();16];

        let slots=Slots {
            cursor:ObjectMeshID::zeroed(),
            cursor_a:ObjectMeshID::zeroed(),
            cursor_b:ObjectMeshID::zeroed(),
            tile:ObjectMeshID::zeroed(),
            terrain_textures,
            floor_mesh:TerrainMeshID::new(ID::zeroed()),
            wall_meshes,
            hole_meshes
        };

        slots
        */
    }

    pub fn set_slot(&mut self, set_slot:SetSlot) {
    }
}