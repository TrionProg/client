use std;

use reactor;

use types::*;
use consts::*;

use gfx;
use gfx::Device;

use gfx_gl;
use gfx_gl::Device;

use glutin;
use glutin::GlContext;
use glutin::EventsLoop;

use gfx_glutin;

use std::thread;
use std::thread::JoinHandle;

use supervisor;
use supervisor::SupervisorSender;
use supervisor::SupervisorCommand;

use controller;
use controller::ControllerSender;
use controller::ControllerCommand;

use process;
use process::ProcessSender;
use process::ProcessCommand;

use ::Camera as CommonCamera;

use super::Error;
use super::Window;
use super::Targets;
use super::Storage;
use super::Slots;
use super::RenderCommand;
use super::{LoadTexture, LoadMesh, LoadLod, SetSlot};
use super::{Trace,TracePool};

pub type RenderSender = reactor::Sender<ThreadSource,RenderCommand>;
pub type RenderReceiver = reactor::Receiver<ThreadSource,RenderCommand>;

pub type Encoder = gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>;

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];


pub struct Render {
    render_receiver:RenderReceiver,
    supervisor_sender:SupervisorSender,
    controller_sender:ControllerSender,
    process_sender:ProcessSender,

    window: Window,
    targets:Targets,


    //clear_color: [f32; 4],
    gfx_device: Device,
    encoder: Encoder,
    //pub pso: gfx::PipelineState<gfx_gl::Resources, pipe::Meta>,
    //pso_wire: gfx::PipelineState<gfx_gl::Resources, pipe::Meta>,
    storage: Storage,
    slots: Slots,
    //font: rusttype::Font<'static>,
    //pub data: pipe::Data<gfx_gl::Resources>,

    camera:CommonCamera,
}

impl Render{
    pub fn run()-> (JoinHandle<()>, RenderSender) {
        let (render_sender, mut render_receiver) = reactor::create_channel(ThreadSource::Render);

        let join_handle=thread::Builder::new().name("Render".to_string()).spawn(move|| {
            let (mut supervisor_sender, mut controller_sender, mut process_sender) = Self::get_senders(&mut render_receiver).unwrap();

            let camera=wait![render_receiver,
                RenderCommand::Camera(camera) => camera
            ].unwrap();

            println!("R1");

            let mut events_loop = glutin::EventsLoop::new();

            let mut render=match Self::setup(
                render_receiver,
                supervisor_sender.clone(),
                controller_sender.clone(),
                process_sender.clone(),
                &events_loop,
                camera
            ) {
                Ok(render) => render,
                Err(error) => {
                    println!("Render setup error: {}", error);

                    send![
                        supervisor_sender, SupervisorCommand::ThreadCrash(ThreadSource::Render),
                        controller_sender, ControllerCommand::ThreadCrash(ThreadSource::Render),
                        process_sender, ProcessCommand::ThreadCrash(ThreadSource::Render)
                    ].unwrap();

                    return;
                }
            };

            println!("R2");


            send![
                 controller_sender, ControllerCommand::EventsLoop(events_loop)
            ].unwrap();

            render.synchronize_setup().unwrap();

            println!("R3");

            match render.lifecycle() {
                Ok(_) => {
                    //do something

                    println!("R4");

                    render.synchronize_finish().unwrap();
                }
                Err(error) => {
                    println!("Render Error: {}!", error);

                    match error {
                        Error::ThreadCrash(_,thread) => {
                            /*
                            if source==ThreadSource::Disk {
                                try_send![disk.storage_sender, StorageCommand::IpcListenerThreadCrash(source)];
                            }
                            */
                        }
                        _ => {
                            send![
                                supervisor_sender , SupervisorCommand::ThreadCrash(ThreadSource::Render),
                                controller_sender , ControllerCommand::ThreadCrash(ThreadSource::Render),
                                process_sender , ProcessCommand::ThreadCrash(ThreadSource::Render)
                            ].unwrap();
                        }
                    }
                }
            }

            println!("R5");
        }).unwrap();

        (join_handle, render_sender)
    }

    fn get_senders(receiver:&mut RenderReceiver) -> Result<(SupervisorSender, ControllerSender, ProcessSender),Error> {
        let supervisor_sender=wait![receiver,
            RenderCommand::SupervisorSender(supervisor_sender) => supervisor_sender
        ].unwrap();

        let controller_sender=wait![receiver,
            RenderCommand::ControllerSender(controller_sender) => controller_sender
        ].unwrap();

        let process_sender=wait![receiver,
            RenderCommand::ProcessSender(process_sender) => process_sender
        ].unwrap();

        ok!((supervisor_sender, controller_sender, process_sender))
    }

    fn setup(
        render_receiver:RenderReceiver,
        supervisor_sender:SupervisorSender,
        controller_sender:ControllerSender,
        process_sender:ProcessSender,
        events_loop:&EventsLoop,
        camera:CommonCamera
    ) -> Result<Self,Error> {
        let window_config = glutin::WindowBuilder::new()
            .with_title(WINDOW_TITLE.to_string())
            .with_dimensions(1024, 768);//TODO from config
        let context = glutin::ContextBuilder::new()
            .with_vsync(true);

        let (
            gfx_window,
            gfx_device,
            mut gfx_factory,
            final_color_target_view,
            final_depth_target_view
        ) = gfx_glutin::init::<super::targets::FinalColorFormat, super::targets::FinalDepthFormat>(window_config, context, events_loop);

        let window=Window::new(gfx_window, 1024, 768);

        let targets=Targets {
            final_color:final_color_target_view,
            final_depth:final_depth_target_view
        };

        let storage=Storage::new(gfx_factory.clone())?;

        let mut encoder: gfx::Encoder<_, _> = gfx_factory.create_command_buffer().into();

        let render=Render {
            render_receiver,
            supervisor_sender,
            controller_sender,
            process_sender,

            window,
            targets,

            gfx_device,
            encoder,
            storage,
            slots:Slots::new(),

            camera
        };

        ok!(render)
    }

    fn synchronize_setup(&mut self) -> Result<(),Error>{
        try_send![self.supervisor_sender, SupervisorCommand::ThreadReady(ThreadSource::Render)];

        wait![self.render_receiver,
            RenderCommand::SupervisorReady => ()
        ].unwrap();

        ok!()
    }

    fn lifecycle(&mut self) -> Result<(),Error> {
        loop {
            self.render()?;

            if self.handle_render_commands()? {
                return ok!();
            }
        }
    }

    fn handle_render_commands(&mut self) -> Result<bool,Error> {
        loop {
            match try_recv_block!(self.render_receiver) {
                RenderCommand::ThreadCrash(thread) => return err!(Error::ThreadCrash, thread),
                RenderCommand::Tick => return ok!(false),
                RenderCommand::Shutdown => return ok!(true),

                RenderCommand::ResizeWindow(width, height) =>
                    self.window.resize(width, height, &mut self.targets),

                RenderCommand::StorageCommand(command) =>
                    self.storage.process_command(command)?,

                RenderCommand::SetSlot(set_slot) =>
                    self.slots.set_slot(set_slot),


/*
                RenderCommand::ResourcesReady => {
                    self.resources_loaded=true;
                    try_send!(self.process_sender, ProcessCommand::ResourcesLoaded);
                },
                */

                _ => unreachable!()
            }
        }
    }

    fn render(&mut self) -> Result<(),Error> {
        self.gfx_device.cleanup();
        self.encoder.clear(&self.targets.final_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.targets.final_depth, 1.0);

        if self.resources_loaded {
            self.render_map()?;
        }

        self.encoder.flush(&mut self.gfx_device);

        self.window.swap_buffers()?;

        ok!()
    }

    fn render_map(&mut self) -> Result<(),Error> {
        /*
        //use storage::mesh::MeshID;
        //use gfx::traits::FactoryExt;
        //use gfx::Factory;
        //use object_pool::growable::ID;
        //use cgmath::SquareMatrix;
        //use gfx::texture::SamplerInfo;

        let camera=self.camera.get_render_camera()?.unwrap();
        let proj_view_matrix=camera.perspective_matrix * camera.camera_matrix;

        self.encoder.update_constant_buffer(
            &self.storage.object_globals,
            &super::pipelines::object::ObjectGlobals {
                proj_view_matrix: proj_view_matrix.into()
            },
        );

        self.encoder.update_constant_buffer(
            &self.storage.trace_globals,
            &super::pipelines::trace::TraceGlobals {
                proj_view_matrix: proj_view_matrix.into()
            },
        );

        match self.map {
            Some(ref map) => {
                for z in 0..MAP_SIZE {
                    for x in 0..MAP_SIZE {
                        match map.tiles[x][z] {
                            Tile::Air => {},
                            Tile::Floor(index) => {
                                let mesh_id=self.slots.floor_mesh;
                                let texture_id=self.slots.terrain_textures[index];

                                self.storage.terrain_meshes.get(mesh_id)?.draw(
                                    &self.storage, &mut self.encoder, &self.targets,
                                    x as u32,z as u32,texture_id
                                )?;
                            }
                            Tile::Wall(index) => {
                                let r=if x<MAP_SIZE-1 && map.tiles[x+1][z].is_wall() {0}else{1<<0};
                                let l=if x>0 && map.tiles[x-1][z].is_wall() {0}else{1<<1};
                                let f=if z<MAP_SIZE-1 && map.tiles[x][z+1].is_wall() {0}else{1<<2};
                                let b=if z>0 && map.tiles[x][z-1].is_wall() {0}else{1<<3};

                                let mask=r | l | f | b;

                                let mesh_id=self.slots.wall_meshes[mask];
                                let texture_id=self.slots.terrain_textures[index];

                                self.storage.terrain_meshes.get(mesh_id)?.draw(
                                    &self.storage, &mut self.encoder, &self.targets,
                                    x as u32,z as u32,texture_id
                                )?;
                            },
                            Tile::Hole(index) => {
                                let r=if x<MAP_SIZE-1 && map.tiles[x+1][z].is_hole() {0}else{1<<0};
                                let l=if x>0 && map.tiles[x-1][z].is_hole() {0}else{1<<1};
                                let f=if z<MAP_SIZE-1 && map.tiles[x][z+1].is_hole() {0}else{1<<2};
                                let b=if z>0 && map.tiles[x][z-1].is_hole() {0}else{1<<3};

                                let mask=r | l | f | b;

                                let mesh_id=self.slots.hole_meshes[mask];
                                let texture_id=self.slots.terrain_textures[index];

                                self.storage.terrain_meshes.get(mesh_id)?.draw(
                                    &self.storage, &mut self.encoder, &self.targets,
                                    x as u32,z as u32,texture_id
                                )?;
                            },
                        }
                    }
                }
            },
            None => {},
        }

        //CursorA
        match self.cursor_a {
            Some((x,z)) => {
                let mesh_id=self.slots.cursor_a;
                self.storage.object_meshes.get(mesh_id)?.draw(
                    &self.storage, &mut self.encoder, &self.targets,
                    x, 0.05, z,
                )?;
            },
            None => {},
        }

        //CursorB
        match self.cursor_b {
            Some((x,z)) => {
                let mesh_id=self.slots.cursor_b;
                self.storage.object_meshes.get(mesh_id)?.draw(
                    &self.storage, &mut self.encoder, &self.targets,
                    x, 0.05, z,
                )?;
            },
            None => {},
        }

        //Tiles
        for &(x,z) in self.tiles.iter() {
            let mesh_id=self.slots.tile;
            self.storage.object_meshes.get(mesh_id)?.draw(
                &self.storage, &mut self.encoder, &self.targets,
                x, 0.025,z
            )?;
        }

        //CursorTile
        match self.cursor_tile {
            Some((x,z)) => {
                let mesh_id=self.slots.cursor;
                self.storage.object_meshes.get(mesh_id)?.draw(
                    &self.storage, &mut self.encoder, &self.targets,
                    x, 0.04, z,
                )?;
            },
            None => {},
        }

        //Cursor
        let mesh_id=self.slots.cursor;
        self.storage.object_meshes.get(mesh_id)?.draw(
            &self.storage, &mut self.encoder, &self.targets,
            self.cursor_pos.0, 0.1,self.cursor_pos.1,
        )?;

        self.traces.draw(&self.storage, &mut self.encoder, &self.targets)?;
        */
        ok!()
    }

    fn synchronize_finish(&mut self) -> Result<(),Error>{
        println!("R F1");
        try_send![self.supervisor_sender, SupervisorCommand::ThreadFinished(ThreadSource::Render)];

        wait![self.render_receiver,
            RenderCommand::SupervisorFinished => ()
        ].unwrap();

        println!("R F");

        ok!()
    }
}