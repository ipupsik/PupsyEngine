use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

use ash::vk;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use crate::vk::constants;
use crate::utility::platforms;
use crate::utility::debug;
use crate::utility::tools;

pub struct ValidationInfo {
    pub is_enable: bool,
    pub required_validation_layers: [&'static str; 1],
}

pub fn setup_debug_utils(entry: &ash::Entry, instance: &ash::Instance) -> (ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT){
    let debug_units_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

    if constants::VALIDATION.is_enable == false {
        (debug_units_loader, ash::vk::DebugUtilsMessengerEXT::null())
    }
    else {
        let messanger_ci = debug_messanger_create_info();

        let debug_utils_messanger = unsafe {
            debug_units_loader
            .create_debug_utils_messenger(&messanger_ci, None)
            .expect("Failed to create debug utils messanger")
        };

        (debug_units_loader, debug_utils_messanger)
    }
}

fn debug_messanger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            // vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            // vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(debug::vulkan_debug_utils_callback),
        p_user_data: ptr::null_mut(),
        _marker: PhantomData
    }
}

pub fn check_validation_layer_support(
    entry: &ash::Entry
) -> bool {
    let layer_properties = entry
        .enumerate_instance_layer_properties()
        .expect("Failed to enumerate Instance Layers Properties");

    for required_layer_name in constants::VALIDATION.required_validation_layers.iter() {
        let mut is_layer_found = false;

        for layer_property in layer_properties.iter() {
            let test_layer_name = tools::vk_to_string(&layer_property.layer_name);
            if (*required_layer_name) == test_layer_name {
                is_layer_found = true;
                break;
            }
        }

        if is_layer_found == false {
            return false;
        }
    }

    layer_properties.len() > 0
}