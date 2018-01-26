
use location::{Pos2D,Pos3D};

gfx_defines! {
    vertex P3N3TcVertex {
        pos: [f32; 3] = "a_pos",
        normal: [f32; 3] = "a_normal",
        uv: [f32; 2] = "a_uv",
    }
}

impl P3N3TcVertex {
    pub fn new(pos:Pos3D, normal:Pos2D, uv:Pos2D) -> Self {
        P3N3TcVertex {
            pos: [pos.x, pos.y, pos.z],
            normal: [normal.x, normal.y, normal.z],
            uv: [uv.x, uv.y]
        }
    }
}
