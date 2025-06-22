use super::*;

//////////
// Mesh //
//////////

// todo: make constructor

pub struct Mesh 
{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub pos: cgmath::Vector3<f32>,
    pub tex_loc: String,
    pub drawable: Drawable,
}

pub struct Drawable
{
    pub to_world: cgmath::Matrix4<f32>,
    pub data: pipe::Data<Resources>,
    pub slice: gfx::Slice<Resources>,
}

impl Mesh
{
    pub fn translate(&mut self, by: cgmath::Vector3<f32>)
    {
        self.pos += by;
        self.drawable.to_world = cgmath::Matrix4::from_translation(self.pos);
    }
}

impl specs::Component for Mesh
{
    type Storage = specs::VecStorage<Mesh>;
}

////////////
// Player //
////////////

pub struct Player;

impl specs::Component for Player
{
    type Storage = specs::VecStorage<Player>;
}

////////////
// Camera //
////////////

pub struct Camera
{
    pub position: cgmath::Point3<f32>,
    pub look_at: cgmath::Point3<f32>,
    up_direction: cgmath::Vector3<f32>,
    perspective: cgmath::Matrix4<f32>,
}

impl Camera
{
    pub fn new
    (
        position: cgmath::Point3<f32>,
        look_at: cgmath::Point3<f32>,
        up_direction: cgmath::Vector3<f32>,
        fov: f32, aspect_ratio: f32, near_clip: f32, far_clip: f32,
    ) -> Camera
    {
        Camera
        {
            position: position,
            look_at: look_at,
            up_direction: up_direction,
            perspective: cgmath::perspective(cgmath::Deg(fov), aspect_ratio, near_clip, far_clip),
        }
    }

    pub fn get_matrix(&self) -> cgmath::Matrix4<f32>
    {
        let view_point = cgmath::Matrix4::look_at
        (
            self.position,
            self.look_at,
            self.up_direction, //unit_z
        );

        (self.perspective * view_point).into()
    }
}