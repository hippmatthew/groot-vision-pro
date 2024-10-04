mod queuefamilies;

use ash::{khr, vk};
use queuefamilies::*;

pub struct Device {
    vk_physicalDevice: vk::PhysicalDevice,
    vk_device: vk::Device,
    queue_families: QueueFamilies,
    extensions: Vec<*const i8>
}

impl Device {
    pub fn new(
        vk_instance: &ash::Instance,
        surface_loader: &khr::surface::Instance,
        vk_surface: &vk::SurfaceKHR,
    ) {
    }

    fn get_physical_device(
        vk_instance: &ash::Instance,
        surface_loader: &khr::surface::Instance,
        vk_surface: &vk::SurfaceKHR,
    ) -> vk::PhysicalDevice {
        let physical_devices = match unsafe { vk_instance.enumerate_physical_devices() } {
            Ok(devices) => devices,
            Err(error) => panic!("failed to get physical devices with error: {error}"),
        };

        // conditions for a suitable gpu:
        // 1. must have an all queue
        // 2. must support all required extensions
        // 3. must have viable surface formats / present modes

        for physical_device in physical_devices {
            let queue_families =
                QueueFamilies::get(vk_instance, surface_loader, vk_surface, &physical_device);

            if !queue_families.has_family(QueueFamilyType::All) {
                continue;
            }

            let extensions = [ vk::KHR_SURFACE_NAME.as_ptr(), vk::KHR_SWAPCHAIN_NAME.as_ptr(), vk::KHR_DYNAMIC_RENDERING_NAME.as_ptr(), ]
        }

        vk::PhysicalDevice::default()
    }
}
