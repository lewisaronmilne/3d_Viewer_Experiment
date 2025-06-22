use super::*;

pub struct Responses
{
    response_receiver: mpsc::Receiver<Response>
}

impl Responses 
{
    pub fn new(response_receiver: mpsc::Receiver<Response>) -> Responses
    {
        Responses
        {
            response_receiver: response_receiver
        }
    }
}

impl specs::System<()> for Responses
{
    fn run(&mut self, arg: specs::RunArg, _: ())
    {
        arg.fetch(|w| 
        {
            for response in self.response_receiver.try_iter()
            {
                match response
                {
                    Response::AddMesh(mesh) => 
                    {
                        w.create().with(mesh).build();
                    },
                    Response::AddPlayer(mesh) => 
                    {
                        let player = components::Player;
                        w.create().with(mesh).with(player).build();
                    },
                };
            }
        });
    }
}