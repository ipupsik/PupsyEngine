use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

use crate::utility::constants;

pub struct Window {
    pub window: winit::window::Window,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Window {
        let window = winit::window::WindowBuilder::new()
            .with_title(constants::WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.");

        Window{
            window: window
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}