use ash::vk;
use crate::vk::debug;

pub const API_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);

pub const VALIDATION: debug::ValidationInfo = debug::ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};