#![allow(unused_must_use)]

#[macro_use] extern crate gfx;
extern crate glutin as window_face;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate cgmath;
extern crate image;
extern crate serde_json;
extern crate specs;

#[macro_use] extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_sys;

mod engine;
mod miscs;

static ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");
static VARS: engine::Vars = engine::Vars
{
    cam_sensitivity: 1.0,
    cam_distance: 40.0,
};

fn main()
{
    engine::start();
}