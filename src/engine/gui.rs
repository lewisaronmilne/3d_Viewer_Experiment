use super::*;

use imgui;
use imgui_gfx_renderer;

//////////////////
// Construction //
//////////////////

fn construct_gui(frame: &imgui::Ui)
{
    frame.window(im_str!("Hello world"))
        .position((10.0, 10.0), imgui::ImGuiSetCond_FirstUseEver)
        .size((300.0, 100.0), imgui::ImGuiSetCond_FirstUseEver)
        .build(||
        {
            frame.text(im_str!("Hello world!"));
            frame.text(im_str!("This...is...imgui-rs!"));
            frame.separator();
            let mouse_pos = frame.imgui().mouse_pos();
            frame.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        });

    frame.window(im_str!("Hello worlratsratsd"))
        .position((10.0, 120.0), imgui::ImGuiSetCond_FirstUseEver)
        .size((300.0, 50.0), imgui::ImGuiSetCond_FirstUseEver)
        .build(||
        {
            frame.text(im_str!("I think understand how to use it!"));
        });
}

///////////////
// Interface //
///////////////

pub struct Gui 
{
    imgui: imgui::ImGui,
    renderer: imgui_gfx_renderer::Renderer<Resources>,
    mouse_pos: (i32, i32),
    mouse_pressed: (bool, bool, bool),
    mouse_wheel: f32,
}

impl Gui
{
    pub fn new(factory: &mut Factory, render_target: RenderHandle) -> Gui
    {
        use imgui::{ImGui, ImGuiKey};

        let mut imgui = ImGui::init();

        imgui.set_imgui_key(ImGuiKey::Tab, 0);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
        imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
        imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
        imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
        imgui.set_imgui_key(ImGuiKey::PageUp, 5);
        imgui.set_imgui_key(ImGuiKey::PageDown, 6);
        imgui.set_imgui_key(ImGuiKey::Home, 7);
        imgui.set_imgui_key(ImGuiKey::End, 8);
        imgui.set_imgui_key(ImGuiKey::Delete, 9);
        imgui.set_imgui_key(ImGuiKey::Backspace, 10);
        imgui.set_imgui_key(ImGuiKey::Enter, 11);
        imgui.set_imgui_key(ImGuiKey::Escape, 12);
        imgui.set_imgui_key(ImGuiKey::A, 13);
        imgui.set_imgui_key(ImGuiKey::C, 14);
        imgui.set_imgui_key(ImGuiKey::V, 15);
        imgui.set_imgui_key(ImGuiKey::X, 16);
        imgui.set_imgui_key(ImGuiKey::Y, 17);
        imgui.set_imgui_key(ImGuiKey::Z, 18);

        let renderer = imgui_gfx_renderer::Renderer::init(&mut imgui, factory, render_target);

        Gui
        {
            imgui: imgui,
            mouse_pos: (0, 0),
            mouse_pressed: (false, false, false),
            mouse_wheel: 0.0,
            renderer: renderer,
        }
    }

    pub fn render(&mut self, factory: &mut Factory, encoder: &mut Encoder, window: &window_face::Window)
    {
        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();
        let frame = self.imgui.frame(size_points, size_pixels, 1.0 / 16.0); //magic

        construct_gui(&frame);

        self.renderer.render(frame, factory, encoder);
    }

    pub fn check_event(&mut self, event: &window_face::Event) 
    {
        use window_face::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, TouchPhase};
        
        match *event 
        {
            Event::KeyboardInput(state, _, code) => 
            {
                let pressed = state == ElementState::Pressed;
                match code 
                {
                    Some(VirtualKeyCode::Tab) => self.imgui.set_key(0, pressed),
                    Some(VirtualKeyCode::Left) => self.imgui.set_key(1, pressed),
                    Some(VirtualKeyCode::Right) => self.imgui.set_key(2, pressed),
                    Some(VirtualKeyCode::Up) => self.imgui.set_key(3, pressed),
                    Some(VirtualKeyCode::Down) => self.imgui.set_key(4, pressed),
                    Some(VirtualKeyCode::PageUp) => self.imgui.set_key(5, pressed),
                    Some(VirtualKeyCode::PageDown) => self.imgui.set_key(6, pressed),
                    Some(VirtualKeyCode::Home) => self.imgui.set_key(7, pressed),
                    Some(VirtualKeyCode::End) => self.imgui.set_key(8, pressed),
                    Some(VirtualKeyCode::Delete) => self.imgui.set_key(9, pressed),
                    Some(VirtualKeyCode::Back) => self.imgui.set_key(10, pressed),
                    Some(VirtualKeyCode::Return) => self.imgui.set_key(11, pressed),
                    Some(VirtualKeyCode::Escape) => self.imgui.set_key(12, pressed),
                    Some(VirtualKeyCode::A) => self.imgui.set_key(13, pressed),
                    Some(VirtualKeyCode::C) => self.imgui.set_key(14, pressed),
                    Some(VirtualKeyCode::V) => self.imgui.set_key(15, pressed),
                    Some(VirtualKeyCode::X) => self.imgui.set_key(16, pressed),
                    Some(VirtualKeyCode::Y) => self.imgui.set_key(17, pressed),
                    Some(VirtualKeyCode::Z) => self.imgui.set_key(18, pressed),
                    Some(VirtualKeyCode::LControl) | Some(VirtualKeyCode::RControl) => self.imgui.set_key_ctrl(pressed),
                    Some(VirtualKeyCode::LShift) | Some(VirtualKeyCode::RShift) => self.imgui.set_key_shift(pressed),
                    Some(VirtualKeyCode::LAlt) | Some(VirtualKeyCode::RAlt) => self.imgui.set_key_alt(pressed),
                    Some(VirtualKeyCode::LWin) | Some(VirtualKeyCode::RWin) => self.imgui.set_key_super(pressed),
                    _ => {},
                }
            },
            Event::MouseMoved(x, y) => self.mouse_pos = (x, y),
            Event::MouseInput(state, MouseButton::Left) => { self.mouse_pressed.0 = state == ElementState::Pressed },
            Event::MouseInput(state, MouseButton::Right) => { self.mouse_pressed.1 = state == ElementState::Pressed },
            Event::MouseInput(state, MouseButton::Middle) => { self.mouse_pressed.2 = state == ElementState::Pressed },
            Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) => { self.mouse_wheel = y },
            Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => { self.mouse_wheel = y },
            Event::ReceivedCharacter(c) => self.imgui.add_input_character(c),
            _ => (),
        }
    }

    pub fn update_mouse(&mut self) 
    {
        let scale = self.imgui.display_framebuffer_scale();
        self.imgui.set_mouse_pos(self.mouse_pos.0 as f32 / scale.0, self.mouse_pos.1 as f32 / scale.1);
        self.imgui.set_mouse_down(&[self.mouse_pressed.0, self.mouse_pressed.1, self.mouse_pressed.2, false, false]);
        self.imgui.set_mouse_wheel(self.mouse_wheel / scale.1);
        self.mouse_wheel = 0.0;
    }
}
