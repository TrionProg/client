#[macro_use]
extern crate failure;

#[macro_use]
extern crate reactor;

#[macro_use]
extern crate gfx;

extern crate object_pool;
extern crate gfx_window_glutin as gfx_glutin;
extern crate gfx_device_gl as gfx_gl;
extern crate cgmath;
extern crate glutin;
extern crate image;

pub mod types;
pub mod consts;

pub mod location;


pub mod resources;

pub mod storage;
pub mod render;

use storage::{Storage};

fn main() {
    let (render_sender,render_receiver)=reactor::create_channel(types::ThreadSource::Process);
    Storage::initialize(render_sender);
    //println!("{}",get_storage().get_a());
    Storage::delete();
    println!("Hello, world!");
}
