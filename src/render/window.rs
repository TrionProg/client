use gfx_glutin;

use glutin;
use glutin::{GlContext,GlWindow};

use failure::Error;

use super::RenderError;
use super::Targets;


pub struct Window {
    window: GlWindow,
    width:u32,
    height:u32
}

impl Window {
    pub fn new(window:GlWindow, width:u32, height:u32) -> Self {
        Window {
            window,
            width,
            height
        }
    }

    pub fn swap_buffers(&mut self) -> Result<(),Error> {
        match self.window.swap_buffers() {
            Ok(_) => ok!(),
            Err(_) => bail!(RenderError::SwapBuffersError)
        }
    }

    pub fn resize(&mut self, width:u32, height:u32, targets:&mut Targets) {
        self.window.resize(width,height);
        gfx_glutin::update_views(&self.window, &mut targets.final_color, &mut targets.final_depth);

        self.width=width;
        self.height=height;
    }
}