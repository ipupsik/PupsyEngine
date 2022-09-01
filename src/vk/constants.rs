use ash::vk;
use crate::vk::debug;
use std::ffi::CStr;

pub const API_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);

pub const VALIDATION: debug::ValidationInfo = debug::ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};

pub struct DeviceExtension {
    pub names: [&'static str; 1]
}

pub const DEVICE_EXTENSIONS: DeviceExtension = DeviceExtension {
    names: ["VK_KHR_swapchain"],
};