
pub mod error;
pub use self::error::PipelineError;

pub mod consts;
pub mod vertices;

pub mod object;
pub use self::object::{ObjectPipeline, ObjectPSO, create_object_pso};

