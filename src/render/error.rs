
use gfx;
use gfx::shade::ProgramError;

use glutin;

#[derive(Debug, Fail)]
pub enum RenderError {
    #[fail(display = "Swap buffers error : {0}", _0)]
    SwapBuffersError(glutin::ContextError),
}