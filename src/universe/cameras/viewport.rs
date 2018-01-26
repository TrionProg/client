
use cgmath::PerspectiveFov;
use location::{Matrix4,Rad};

pub struct Viewport{
    width:u32,
    height:u32,
    perspective_matrix:Matrix4,
}

impl Viewport{
    pub fn configure(width:u32, height:u32) -> Option<Self>{
        if width==0 || height==0 {
            return None;
        }

        let aspect_ratio=width as f32 / height as f32;

        let perspective=PerspectiveFov{
            fovy:Rad(0.5),
            aspect:aspect_ratio,
            near:0.1,
            far:1000.0,
        };

        let perspective_matrix=Matrix4::from(perspective);

        let viewport=Viewport{
            width,
            height,
            perspective_matrix,
        };

        Some(viewport)
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_perspective_matrix(&self) -> &Matrix4 {
        &self.perspective_matrix
    }
}
}
