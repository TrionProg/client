
use render;

use failure::Error;

use location::Location;
use universe::Instance;

pub trait Camera : Instance {
    fn is_enabled(&self) -> Result<bool,Error>;
    fn enable(&self) -> Result<(),Error>;
    fn disable(&self) -> Result<(),Error>;
    fn resize(&self, viewport_width:u32, viewport_height:u32) -> Result<(),Error>;
    fn get_render_camera(&self) -> Result<Option<render::Camera>,Error>;
}