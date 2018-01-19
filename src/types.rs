use std;

pub use std::time::SystemTime as Time;
pub use std::time::Duration;

pub use image::{GrayImage, GrayAlphaImage, RgbImage, RgbaImage};

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum ThreadSource{
    Supervisor=0,
    Render=1,
    Process=2,
    Controller=3
}

impl std::fmt::Display for ThreadSource{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            ThreadSource::Supervisor => write!(f, "Supervisor"),
            ThreadSource::Render => write!(f, "Render"),
            ThreadSource::Process => write!(f, "Process"),
            ThreadSource::Controller => write!(f, "Controller")
        }
    }
}

impl ::reactor::ThreadTrait for ThreadSource{}