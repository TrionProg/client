
use gfx;
use gfx::shade::ProgramError;

#[derive(Debug, Fail)]
pub enum PipelineError {
    #[fail(display = "Link shader error : {}", _0)]
    LinkError(ProgramError),
    #[fail(display = "Create PSO error : {}", _0)]
    CreatePSOError(String)
}