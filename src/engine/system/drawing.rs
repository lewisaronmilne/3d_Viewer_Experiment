use super::*;

pub struct Drawing
{
    pso: Pso,
    render_target: RenderHandle,
    depth_target: DepthHandle,
    encoder_receiver: mpsc::Receiver<Encoder>,
    encoder_sender: mpsc::Sender<Encoder>,
    clear_color: [f32; 4],
}

impl Drawing
{
    pub fn new
    (
        pso: Pso,
        render_target: RenderHandle,
        depth_target: DepthHandle,
        encoder_receiver: mpsc::Receiver<Encoder>,
        encoder_sender: mpsc::Sender<Encoder>,
        clear_color: [f32; 4],
    ) -> Drawing
    {
        Drawing
        {
            pso: pso,
            render_target: render_target,
            depth_target: depth_target,
            encoder_receiver: encoder_receiver,
            encoder_sender: encoder_sender,
            clear_color: clear_color,
        }
    }
}

impl specs::System<()> for Drawing
{
    fn run(&mut self, arg: specs::RunArg, _: ())
    {
        let mut encoder = self.encoder_receiver.recv().unwrap();

        use specs::Join;

        let (meshes, camera) = arg.fetch(|w| 
        {
            (w.read::<components::Mesh>(), w.read_resource::<components::Camera>())
        });

        encoder.clear(&self.render_target, self.clear_color);
        encoder.clear_depth(&self.depth_target, 1.0);

        let cam_matrix = camera.get_matrix();

        for mesh in meshes.join()
        {
            let locals = Locals{ transform: (cam_matrix * mesh.drawable.to_world).into() };
            encoder.update_constant_buffer(&mesh.drawable.data.locals, &locals);
            encoder.draw(&mesh.drawable.slice, &self.pso, &mesh.drawable.data);
        }

        self.encoder_sender.send(encoder);
    }
}