use ash::vk;

pub const APPLICATION_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);
pub const ENGINE_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub const WINDOW_TITLE: &'static str = "Pupsy Window";
pub const ENGINE_TITLE: &'static str = "Pupsy Engine";