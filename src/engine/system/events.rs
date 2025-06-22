use super::*;

pub struct Events
{
    event_receiver: mpsc::Receiver<Event>,
    request_sender: mpsc::Sender<Request>, 
    cam_angle: (f32, f32),
}

impl Events {
    pub fn new(event_receiver: mpsc::Receiver<Event>, request_sender: mpsc::Sender<Request>) -> Events
    {
        Events
        {
            event_receiver: event_receiver,
            request_sender: request_sender,
            cam_angle: (0.0, 0.0),
        }
    }
}

impl specs::System<()> for Events
{
    fn run(&mut self, arg: specs::RunArg, _: ())
    {
        let (mut camera, mut meshes, players) = arg.fetch(|w| 
        {
            (w.write_resource::<components::Camera>(), w.write::<components::Mesh>(), w.read::<components::Player>())
        });

        for event in self.event_receiver.try_iter()
        {
            use window_face::VirtualKeyCode::*;

            let mut move_cube = |mut theta: f32|
            {
                if theta < 0.0 { theta = theta + 6.28319; }
                if theta > 6.28318 { theta = theta - 6.28319; }
                let pos = 0.5 * cgmath::Vector3::new(-theta.cos(), -theta.sin(), 0.0);

                use specs::Join;
                for (m, _) in (&mut meshes, &players).join()
                {
                    m.translate(pos);
                };
            };

            match event 
            {
                Event::MouseDelta(x, y) =>
                {
                    let mut theta = self.cam_angle.0 + (x as f32 / 1000.0 * ::VARS.cam_sensitivity);
                    let mut phi = self.cam_angle.1 - (y as f32 / 1000.0 * ::VARS.cam_sensitivity);

                    if theta < 0.0 { theta = theta + 6.28319; }
                    if theta > 6.28318 { theta = theta - 6.28319; }

                    if phi < 0.001 { phi = 0.001; }
                    if phi > 3.14159 { phi = 3.14159; }

                    self.cam_angle.0 = theta;
                    self.cam_angle.1 = phi;

                    let cam_pivot = 4.0 * cgmath::Point3::new(0.0 as f32, 0.0, 0.0);
                    let cam_offset = ::VARS.cam_distance * cgmath::Point3::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos());
   
                    camera.position = cam_offset + (4.0 * cgmath::Vector3::new(0.0 as f32, 0.0, 0.0));
                    camera.look_at = cam_pivot;
                },
                Event::ButtonPressed(L) | Event::ButtonPressed(Up)    => move_cube(self.cam_angle.0 + 0.0),
                Event::ButtonPressed(A) | Event::ButtonPressed(Left)  => move_cube(self.cam_angle.0 + 1.57080),
                Event::ButtonPressed(T) | Event::ButtonPressed(Down)  => move_cube(self.cam_angle.0 + 3.14159),
                Event::ButtonPressed(S) | Event::ButtonPressed(Right) => move_cube(self.cam_angle.0 + 4.71239),
                _ => {},
            }
        }
    }
}