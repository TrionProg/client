
use render;

use failure::Error;

use reactor::Mutex;

use location::{Pos3D,Matrix4};
use universe::Instance;

use super::Viewport;
use super::Camera;

pub struct ThirdPersonCamera {
    inner:Mutex<InnerCamera>
}

pub struct InnerCamera {
    pub center_position: Pos3D,
    pub pitch:f32,
    pub yaw:f32,
    pub distance:f32,
    camera_matrix:Matrix4,
    camera_position:Pos3D,
    pub enabled:bool,

    pub viewport:Option<Viewport>
}

impl ThirdPersonCamera{
    pub fn new(window_width:u32, window_height:u32) -> Self {
        ThirdPersonCamera {
            inner: Mutex::new(InnerCamera::new(window_width, window_height))
        }
    }

    pub fn edit<F,T>(&self, f:F) -> Result<T,Error> where F:FnOnce(&mut InnerCamera) -> T {
        mutex_lock!(self.inner => camera);

        let t=f(&mut camera)?;
        camera.calc_matrix();

        ok!(t)
    }
}

impl Camera for ThirdPersonCamera {
    fn is_enabled(&self) -> Result<bool,Error> {
        mutex_lock!(self.inner => camera);
        ok!(camera.enabled)
    }

    fn enable(&self) -> Result<(),Error> {
        mutex_lock!(self.inner => camera);
        camera.enabled=true;

        ok!()
    }

    fn disable(&self) -> Result<(),Error> {
        mutex_lock!(self.inner => camera);
        camera.enabled=false;

        ok!()
    }

    fn resize(&self, viewport_width:u32, viewport_height:u32) -> Result<(),Error> {
        mutex_lock!(self.inner => camera);
        camera.viewport=Viewport::configure(window_width, window_height);

        ok!()
    }

    fn get_render_camera(&self) -> Result<Option<render::Camera>,Error> {
        mutex_lock!(self.inner => camera);

        let perspective_matrix=match camera.viewport {
            Some( ref viewport ) => viewport.get_perspective_matrix().clone(),
            None => return ok!(None),
        };

        let render_camera=render::Camera::new(
            camera.camera_matrix.clone(),
            camera.camera_position.clone(),
            perspective_matrix
        );

        ok!( Some(render_camera) )
    }
}

impl Instance for ThirdPersonCamera {
    fn set_location(&self, location:&Location) -> Result<(),Error> {
        mutex_lock!(self.inner => camera);

        camera.center_position=location.position;
        //TODO angle

        ok!()
    }
}


impl InnerCamera {
    fn new(window_width: u32, window_height: u32) -> Self {
        use cgmath::SquareMatrix;

        let mut camera = InnerCamera {
            center_position: Pos3D::new(0.0, 0.0, 0.0),
            pitch:3.14/4.0,
            yaw:3.14/4.0,
            distance: 10.0,
            camera_matrix: Matrix4::identity(),
            camera_position: Pos3D::new(0.0, 0.0, 0.0),
            enabled:false,

            viewport: Viewport::configure(window_width, window_height),
        };

        camera.calc_matrix();

        camera
    }

    fn calc_matrix(&mut self) {
        use cgmath::ApproxEq;
        use cgmath::Rotation;
        use cgmath::EuclideanSpace;

        let rot_x:Basis3<f32>=Rotation3::from_angle_x(Rad(-self.pitch));
        let rot_y:Basis3<f32>=Rotation3::from_angle_y(Rad(self.yaw));
        let a=rot_x.rotate_vector(vec3(0.0,0.0,self.distance));
        let b=rot_y.rotate_vector(a);

        self.camera_position=Pos3D::from_vec(b+self.center_position.to_vec());
        self.camera_matrix=Matrix4::look_at(self.camera_position, self.center_position, vec3(0.0,1.0,0.0));
    }
}