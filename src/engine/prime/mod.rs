use super::*;

pub struct Prime
{
    device: Device,
    factory: Factory,
    gui: gui::Gui,
    encoder_receiver: mpsc::Receiver<Encoder>,
    encoder_sender: mpsc::Sender<Encoder>,
    requests_receiver: mpsc::Receiver<Request>,
    response_sender: mpsc::Sender<Response>,
    events_sender: mpsc::Sender<Event>,
    pub window: window_face::Window,
    pub render_target: RenderHandle,
    pub depth_target: DepthHandle,
    meshes_directory: String,
    mouse_attatched: bool,
    pub exit_flag: bool,
}

impl Prime
{
    pub fn start() -> 
    (
        Prime,
        Pso, 
        mpsc::Receiver<Encoder>, 
        mpsc::Sender<Encoder>,
        mpsc::Receiver<Response>,
        mpsc::Sender<Request>, 
        mpsc::Receiver<Event>,
    )
    {
        let builder = window_face::WindowBuilder::new().with_title("Super Duper").with_dimensions(1000, 1000).with_vsync();
        let (window, device, mut factory, render_target, depth_target) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

        let (window_width, window_height) = window.get_inner_size_points().unwrap();
        window.set_cursor(window_face::MouseCursor::NoneCursor);
        window.set_cursor_position((window_width/2) as i32, (window_height/2) as i32);
        window.set_cursor_state(window_face::CursorState::Grab); 

        let pso = 
        {
            use gfx::traits::FactoryExt;
            let shader_set = factory.create_shader_set(&::miscs::read_bytes("assets/shaders/vert.glsl"), &::miscs::read_bytes("assets/shaders/frag.glsl")).unwrap();

            factory.create_pipeline_state 
            (
                &shader_set,
                gfx::Primitive::TriangleList, 
                gfx::state::Rasterizer
                {
                    front_face: gfx::state::FrontFace::CounterClockwise,
                    cull_face: gfx::state::CullFace::Back,
                    method: gfx::state::RasterMethod::Fill,
                    offset: None,
                    samples: None,
                },
                pipe::new()
            ).unwrap()
        };

        let gui = gui::Gui::new(&mut factory, render_target.clone());

        let (prime_encoder_sender, sys_encoder_receiver) = mpsc::channel();
        let (sys_encoder_sender, prime_encoder_receiver) = mpsc::channel();
        let (prime_response_sender, sys_response_receiver) = mpsc::channel();
        let (sys_request_sender, prime_request_receiver) = mpsc::channel();
        let (prime_event_sender, sys_event_receiver) = mpsc::channel();

        prime_encoder_sender.send(factory.create_command_buffer().into());
        sys_encoder_sender.send(factory.create_command_buffer().into());

        let mut prime = Prime 
        {
            device: device,
            factory: factory,
            gui: gui,
            encoder_receiver: prime_encoder_receiver,
            encoder_sender: prime_encoder_sender,
            requests_receiver: prime_request_receiver,
            response_sender: prime_response_sender,
            events_sender: prime_event_sender,
            window: window,
            render_target: render_target,
            depth_target: depth_target,
            meshes_directory: "assets/meshes".to_string(),
            mouse_attatched: true,
            exit_flag: false,
        };

        let player_mesh = prime.make_mesh("cube.mesh", cgmath::Vector3::new(0.0, 0.0, 0.0));
        prime.response_sender.send(Response::AddPlayer(player_mesh));

        (prime, pso, sys_encoder_receiver, sys_encoder_sender, sys_response_receiver, sys_request_sender, sys_event_receiver)
    }

    pub fn flush(&mut self)
    {
        loop
        {
            self.flush_events();
            self.flush_requests();
            if self.flush_encoder()
                { break }
        }
    }

    fn flush_encoder(&mut self) -> bool
    {
        let mut encoder = match self.encoder_receiver.try_recv()
        {
            Ok(enc) => enc,
            Err(_) => return false,
        };

        self.gui.render(&mut self.factory, &mut encoder, &self.window);

        encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        use gfx::Device;
        self.device.cleanup();
        self.encoder_sender.send(encoder);

        return true
    }

    fn flush_events(&mut self) 
    {
        for event in self.window.poll_events()
        {
            use window_face::VirtualKeyCode::*;
            use window_face::ElementState::*;
            use window_face::Event::*;
            use engine::Event::*;

            self.gui.check_event(&event);

            match event
            {
                Closed | KeyboardInput(Pressed, _, Some(Escape)) => { self.exit_flag = true; },
                KeyboardInput(Pressed, _, Some(Z)) => 
                {
                    if self.mouse_attatched
                    {
                        self.mouse_attatched = false; 
                        self.window.set_cursor(window_face::MouseCursor::Default);
                        self.window.set_cursor_state(window_face::CursorState::Normal); 
                    }
                    else 
                    {
                        self.mouse_attatched = true; 
                        let (window_width, window_height) = self.window.get_inner_size_points().unwrap();
                        self.window.set_cursor(window_face::MouseCursor::NoneCursor);
                        self.window.set_cursor_position((window_width/2) as i32, (window_height/2) as i32);
                        self.window.set_cursor_state(window_face::CursorState::Grab); 
                    };
                },
                MouseMoved(x, y) => 
                {
                    if self.mouse_attatched
                    {
                        let (window_width, window_height) = self.window.get_inner_size_points().unwrap();
                        let (midpoint_x, midpoint_y) = ((window_width/2) as i32, (window_height/2) as i32);
                        let (mouse_delta_x, mouse_delta_y) = (x - midpoint_x, midpoint_y - y);
                        self.window.set_cursor_position(midpoint_x as i32, midpoint_y as i32);
                        self.events_sender.send(MouseDelta(mouse_delta_x, mouse_delta_y));
                    };
                },
                KeyboardInput(Pressed, _, Some(key)) => { self.events_sender.send(ButtonPressed(key)); }
                _ => {},
            }
        };

        self.gui.update_mouse();
    }

    fn flush_requests(&mut self)
    {
        loop 
        {
            let request = self.requests_receiver.try_recv();
            match request 
            {
                Ok(Request::MakeMesh(mesh_file_loc, pos)) =>
                {
                    let mesh = self.make_mesh(&mesh_file_loc, pos);
                    self.response_sender.send(Response::AddMesh(mesh));
                },
                Err(_) => break
            }
        }
    }

    fn make_mesh(&mut self, mesh_file_loc: &str, pos: cgmath::Vector3<f32>) -> components::Mesh
    {
        use gfx::traits::FactoryExt;

        let cube_data = ::miscs::load_mesh_data(&self.meshes_directory, mesh_file_loc);

        let (vbuf, slice) = self.factory.create_vertex_buffer_with_slice(&cube_data.1 as &[Vertex], &cube_data.2 as &[u32]);
        let locals = self.factory.create_constant_buffer(1);

        let tex_sampler = 
        {
            use image;
            use miscs;

            use gfx::Factory;
            use gfx::texture;
            use std::io::Cursor;

            let data = miscs::read_bytes(&cube_data.0);
            let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
            let (width, height) = img.dimensions();

            let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);

            let (_, shader_resource_view) = self.factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&img]).unwrap();

            let sampler_info = gfx::texture::SamplerInfo::new
            (
                gfx::texture::FilterMethod::Scale,
                gfx::texture::WrapMode::Clamp,
            );

            (shader_resource_view, self.factory.create_sampler(sampler_info))
        };

        let data = pipe::Data 
        {
            vbuf: vbuf,
            locals: locals,
            tex_sampler: tex_sampler,
            render_target: self.render_target.clone(),
            depth_target: self.depth_target.clone(),
        };

        components::Mesh
        {
            vertices: cube_data.1,
            indices: cube_data.2,
            pos: pos,
            tex_loc: cube_data.0,
            drawable: components::Drawable
            {
                to_world: cgmath::Matrix4::from_translation(cgmath::Vector3::new(pos.x, pos.y, pos.z)),
                data: data,
                slice: slice,
            }
        }
    }
}