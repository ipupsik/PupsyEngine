use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

use ash::vk;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use pupsy_engine::vk::constants::{self, VALIDATION};
use pupsy_engine::vk::debug;
use pupsy_engine::vk::render_device;
use pupsy_engine::rhi::window;
use pupsy_engine::rhi;

use pupsy_engine::vk::render_device as vk_render;

struct PupsyEngine {
    render_device: vk_render::VkRenderDevice,
    window: window::Window,
}

impl PupsyEngine {

    pub fn new(window: window::Window) -> PupsyEngine {
        let render_device = vk_render::VkRenderDevice::new(&window);

        PupsyEngine {
             render_device: render_device,
             window: window
        }
    }

    fn draw_frame(&mut self) {
        
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {
            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    self.window.request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                _ => (),
            }

        })
    }
}

impl Drop for PupsyEngine {
    fn drop(&mut self) {
        self.render_device.drop();
    }
}

fn main() {

    let event_loop = EventLoop::new();
    let window = rhi::window::Window::new(&event_loop);

    let engine = PupsyEngine::new(window);
    engine.main_loop(event_loop);
}
