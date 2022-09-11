use crate::rhi::window::Window;

use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use crate::imgui::constants as imgui_constants;

use ash::vk;

pub struct PupsyUiEngine {
    pub imgui:  imgui::Context,
    pub imgui_platform: WinitPlatform,

    dokdo: FontId,
    roboto: FontId,
}

impl PupsyUiEngine {
    pub fn new(window: &Window) -> PupsyUiEngine {
        let mut imgui = Context::create();
        let mut platform = WinitPlatform::init(&mut imgui); 
        platform.attach_window(imgui.io_mut(), &window.window, HiDpiMode::Default);

        let dokdo = imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../../assets/fonts/Dokdo-Regular.ttf"),
            size_pixels: imgui_constants::FONT_SIZE,
            config: None,}]);
    
        let roboto = imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../../assets/fonts/Roboto-Regular.ttf"),
            size_pixels: imgui_constants::FONT_SIZE,
            config: None,
        }]);

        PupsyUiEngine{
            imgui: imgui,
            imgui_platform: platform,
            dokdo: dokdo,
            roboto: roboto
        }
    }

    pub fn render(
        &mut self, window: &Window,
        rhi_cmd : &vk::CommandBuffer) {

        return;

        self.imgui_platform.prepare_frame(self.imgui.io_mut(), &window.window);

        let mut ui = self.imgui.frame();

        let mut run = false;

        imgui::Window::new("Hello world").opened(&mut run).build(&ui, || {
            ui.text("Hello, I'm the default font!");
            let _roboto = ui.push_font(self.roboto);
            ui.text("Hello, I'm Roboto Regular!");
            let _dokdo = ui.push_font(self.dokdo);
            ui.text("Hello, I'm Dokdo Regular!");
            _dokdo.pop();
            ui.text("Hello, I'm Roboto Regular again!");
            _roboto.pop();
            ui.text("Hello, I'm the default font again!");
        });

        self.imgui_platform.prepare_render(&ui, &window.window);

        let ui_draw_data = ui.render();

        

        if !run {
            return;
        }
    }
}
