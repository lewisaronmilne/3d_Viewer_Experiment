use gfx; 
use gfx_device_gl;
use window_face;
use gfx_window_glutin;

use cgmath;
use specs;

use std::sync::mpsc;
use std::thread;

mod prime;
mod system;
mod components;
mod gui;

///////////
// Types //
///////////

pub type Resources = gfx_device_gl::Resources;
pub type Factory = gfx_device_gl::Factory;
pub type Encoder = gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>;
pub type Device = gfx_device_gl::Device;
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type RenderHandle = gfx::handle::RenderTargetView<Resources, ColorFormat>;
pub type DepthHandle = gfx::handle::DepthStencilView<Resources, DepthFormat>;
pub type Pso = gfx::pso::PipelineState<Resources, pipe::Meta>;

/////////////////////
// GFX Definitions //
/////////////////////

gfx_defines!
{
    vertex Vertex 
    {
        pos: [f32; 3] = "a_pos",
        tex_coord: [f32; 2] = "a_tex_coord",
    }

    constant Locals 
    {
        transform: [[f32; 4]; 4] = "u_transform",
    }

    pipeline pipe
    {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "locals",
        tex_sampler: gfx::TextureSampler<[f32; 4]> = "tex_sampler",
        render_target: gfx::RenderTarget<ColorFormat> = "main_window_target",
        depth_target: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

///////////
// Enums //
///////////

pub struct Vars
{
    pub cam_sensitivity: f32,
    pub cam_distance: f32,
}

pub enum Event 
{
    MouseDelta(i32, i32),
    ButtonPressed(window_face::VirtualKeyCode)
}

pub enum Request //prime requests
{
    MakeMesh(String, cgmath::Vector3<f32>),
}

pub enum Response
{
    AddMesh(components::Mesh),
    AddPlayer(components::Mesh),
}

///////////
// Start //
///////////

pub fn start()
{
    let (mut prime, pso, encoder_receiver, encoder_sender, response_receiver, request_sender, event_receiver) = prime::Prime::start();

    let camera = 
    {
        let (window_width, window_height) = prime.window.get_inner_size_points().unwrap();
        components::Camera::new
        (
            cgmath::Point3::new(2.0,2.0,2.0),
            cgmath::Point3::new(0.0, 0.0, 0.0), 
            cgmath::Vector3::unit_z(), 
            60.0, 
            window_width as f32 / window_height as f32, 
            1.0, 
            2000.0,
        )
    };

    let drawing_system = system::drawing::Drawing::new
    (
        pso,
        prime.render_target.clone(),
        prime.depth_target.clone(),
        encoder_receiver,
        encoder_sender,
        [0.1, 0.2, 0.3, 1.0],
    );
    let responses_system = system::responses::Responses::new(response_receiver);
    let events_system = system::events::Events::new(event_receiver, request_sender.clone()); //clone necessary maybe

    thread::spawn(||
    {
        let mut planner = 
        {
            let mut w = specs::World::new();
            w.register::<components::Mesh>();
            w.register::<components::Player>();
            w.add_resource(camera);
            let mut planner = specs::Planner::<()>::new(w);
            planner.add_system(drawing_system, "DRAW_MESHES", 10);
            planner.add_system(events_system, "EVENTS", 9);
            planner.add_system(responses_system, "RESPONSES", 8);
            planner
        };

        loop
        {
            planner.dispatch(());
        }
    });

    while !prime.exit_flag
    {
        prime.flush();
    }
}