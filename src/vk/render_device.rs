use ash::vk;
use ash::vk::ValidationCacheCreateInfoEXT;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use ash;

use std::os::raw::c_char;

use crate::vk::constants;
use crate::utility::constants as global_constants;
use crate::utility::platforms;
use crate::vk::debug;
use crate::utility::tools;

use crate::rhi::render_device;
use crate::rhi::window;

pub struct VkRenderDevice {
    instance: ash::Instance,
    entry: ash::Entry,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messager: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue
}

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

impl render_device::RenderDevice for VkRenderDevice {

}

impl VkRenderDevice
{
    pub fn new (window: &window::Window) -> VkRenderDevice {
        let entry = ash::Entry::default();
        let instance = VkRenderDevice::create_instance(&entry);
        let (debug_units_loader, debug_messager) = debug::setup_debug_utils(&entry, &instance);
        let surface = VkRenderDevice::create_surface(&entry, &instance, window);
        let physical_device = VkRenderDevice::pick_physical_device(&instance, &surface);
        let logical_device = VkRenderDevice::create_logical_device(&instance, physical_device, &constants::VALIDATION, &surface);

        let indices = VkRenderDevice::find_queue_family(&instance, physical_device, &surface);

        let graphics_queue = unsafe { 
            logical_device.get_device_queue(indices.graphics_family.unwrap(), 0)
        };

        let present_queue = unsafe { 
            logical_device.get_device_queue(indices.graphics_family.unwrap(), 0)
        };

        VkRenderDevice {
            entry: entry,
            instance: instance,
            debug_utils_loader: debug_units_loader,
            debug_messager: debug_messager,
            physical_device: physical_device,
            device: logical_device,
            graphics_queue: graphics_queue,
            present_queue: present_queue
        }
    }

    fn create_surface(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &window::Window,
    ) -> VkSurface {
        let surface = unsafe {
            crate::utility::platforms::create_surface(entry, instance, window)
                .expect("Failed to create surface.")
        };
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        VkSurface {
            surface_loader,
            surface,
        }
    }

    pub fn create_instance(entry: &ash::Entry) -> ash::Instance {
        if constants::VALIDATION.is_enable && debug::check_validation_layer_support(entry) == false {
            panic!("Validation layers requested, but not available!");
        }

        let app_name = CString::new(global_constants::WINDOW_TITLE).unwrap();
        let engine_name = CString::new(global_constants::ENGINE_TITLE).unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            p_engine_name: engine_name.as_ptr(),
            application_version: global_constants::APPLICATION_VERSION,
            engine_version: global_constants::ENGINE_VERSION,
            api_version: constants::API_VERSION,
            _marker: std::marker::PhantomData
        };

        let extension_names = platforms::required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
            _marker: std::marker::PhantomData
        };

        let instance: ash::Instance = unsafe {
            entry
            .create_instance(&create_info, None)
            .expect("Failed to created VkInstance!")
        };

        instance
    }

    pub fn pick_physical_device(
        instance: &ash::Instance,
        surface: &VkSurface
    ) -> vk::PhysicalDevice {
        let physical_devices =  unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices")
        };

        println!("{} devoces (GPU) found with Vk support.", physical_devices.len());

        let mut result = None;
        for &physical_device in physical_devices.iter() {
            if VkRenderDevice::is_physical_device_suitable(instance, physical_device, surface) {
                if result.is_none() {
                    result = Some(physical_device);
                }
            }
        }

        result.unwrap()
    }

    fn is_physical_device_suitable(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &VkSurface
    ) -> bool {
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_features = unsafe { instance.get_physical_device_features(physical_device) };
        let device_queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let device_type = match device_properties.device_type {
            vk::PhysicalDeviceType::CPU => "Cpu",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!(),
        };

        let device_name = tools::vk_to_string(&device_properties.device_name);
        println!(
            "\tDevice Name: {}, id: {}, type: {}",
            device_name, device_properties.device_id, device_type
        );

        let major_version = vk::api_version_major(device_properties.api_version);
        let minor_version = vk::api_version_minor(device_properties.api_version);
        let patch_version = vk::api_version_patch(device_properties.api_version);

        println!(
            "\tAPI Version: {}.{}.{}",
            major_version, minor_version, patch_version
        );

        println!("\tSupport Queue Family: {}", device_queue_families.len());
        println!("\t\tQueue Count | Graphics, Compute, Transfer, Sparse Binding");
        for queue_family in device_queue_families.iter() {
            let is_graphics_support = if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                "support"
            } else {
                "unsupport"
            };
            let is_compute_support = if queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                "support"
            } else {
                "unsupport"
            };
            let is_transfer_support = if queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER)
            {
                "support"
            } else {
                "unsupport"
            };
            let is_sparse_support = if queue_family
                .queue_flags
                .contains(vk::QueueFlags::SPARSE_BINDING)
            {
                "support"
            } else {
                "unsupport"
            };

            println!(
                "\t\t{}\t    | {},  {},  {},  {}",
                queue_family.queue_count,
                is_graphics_support,
                is_compute_support,
                is_transfer_support,
                is_sparse_support
            );
        }

        // there are plenty of features
        println!(
            "\tGeometry Shader support: {}",
            if device_features.geometry_shader == 1 {
                "Support"
            } else {
                "Unsupport"
            }
        );

        let indices = VkRenderDevice::find_queue_family(instance, physical_device, surface);

        return indices.is_complete();
    }

    fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &VkSurface,
    ) -> QueueFamilyIndices {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut queue_family_indices = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        };

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_indices.graphics_family = Some(index);
            }

            let is_present_support = unsafe {
                surface
                    .surface_loader
                    .get_physical_device_surface_support(
                        physical_device,
                        index as u32,
                        surface.surface,
                    )
            };

            if queue_family_indices.is_complete() {
                break;
            }

            index += 1;
        }

        queue_family_indices
    }

    pub fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        validation: &debug::ValidationInfo,
        surface: &VkSurface
        ) -> ash::Device {
        let indices = VkRenderDevice::find_queue_family(instance, physical_device, surface);

        let queue_priorities = [1.0_f32];
        let queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: indices.graphics_family.unwrap(),
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,
            _marker: PhantomData
        };

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let required_validation_layer_raw_names: Vec<CString> = validation
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();

        let enable_layer_names: Vec<*const c_char> = required_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_create_info,
            enabled_layer_count: if validation.is_enable {
                enable_layer_names.len()
            } else {
                0
            } as u32,
            pp_enabled_layer_names: if validation.is_enable {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
            p_enabled_features: &physical_device_features,
            _marker: PhantomData
        };

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device!")
        };

        device
    }

    pub fn drop(&mut self) {
        unsafe {
            if constants::VALIDATION.is_enable {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messager, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

pub struct VkSurface {
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
}