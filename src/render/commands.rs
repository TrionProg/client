
use storage::{Resource,ResourceSlot};
use resources::ResourceType;

use failure::Error;

use types::*;

use supervisor::SupervisorSender;
use controller::ControllerSender;
use process::ProcessSender;

use::Camera;

use super::storage::{ObjectMesh,TerrainMesh, TraceMesh};
use super::pipelines::{ObjectVertex, TraceVertex};
use super::Trace;
use super::storage::Storage as RenderStorage;
use super::storage::Resource as RenderResource;
use super::storage::StorageCommandTrait;

pub enum RenderCommand {
    ThreadCrash(ThreadSource),

    SupervisorSender(SupervisorSender),
    ControllerSender(ControllerSender),
    ProcessSender(ProcessSender),
    //Camera(Camera),

    SupervisorReady,
    SupervisorFinished,

    Tick,
    Shutdown,

    ResizeWindow(u32,u32),

    StorageCommand(Box<StorageCommandTrait + Send>),
    SetSlot(SetSlot),
}