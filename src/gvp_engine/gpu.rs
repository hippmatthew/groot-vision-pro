mod queuefamilies;

use queuefamilies::*;

use ash::{vk, khr::surface};

use std::{ffi::CStr, vec::Vec};

pub struct GPU {
  pub device: vk::PhysicalDevice,
  queue_families: QueueFamilyMap
}

impl GPU {
  // Needed methods:
  // 1. get method to automatically acquire gpu
  // 2. methods for accessing queue families
  // 3. query whether the gpu contains a specific queue

  pub fn get(
    instance: &ash::Instance,
    surface_loader: &surface::Instance,
    surface: &vk::SurfaceKHR,
    extensions: &Vec<*const i8>
  ) -> Self {
    // 1. get a list of all gpus
    // 2. loop through all gpus and check if they are suitable
    //      - Must have a main queue
    //      - Must have valid surface formats / present modes
    //      - Must support all required extensions
    // 3. Choose GPU off of priority:
    //      - Physical GPU
    //      - Integrated
    //      - Virtual
    // 4. Store gpu in a variable called best option. Move most suitable gpu into it based on priority
    // 5. Store the queue families

    let gpus = match unsafe { instance.enumerate_physical_devices() } {
      Ok(gpus)    => gpus,
      Err(error)  => panic!("failed to enumerate physical devices with error: {error}")
    };

    let mut device : Option<vk::PhysicalDevice> = None;
    let mut device_type = vk::PhysicalDeviceType::OTHER;
    let mut queue_families : Option<QueueFamilyMap> = None;

    'gpu_loop: for gpu in gpus {
      let properties = unsafe { instance.get_physical_device_properties(gpu) };

      match properties.device_type {
        vk::PhysicalDeviceType::CPU   => continue,
        vk::PhysicalDeviceType::OTHER => continue,
        _ => ()
      }

      if !GPU::has_priority(device_type, properties.device_type) { continue; }

      let map = QueueFamilyMap::populate(instance, surface_loader, surface, &gpu);
      if !map.contains(&QueueFamilyType::Main) { continue; }

      if let Ok(formats) = unsafe { surface_loader.get_physical_device_surface_formats(gpu, *surface) } {
        if formats.is_empty() { continue; }
      } else { continue; }

      if let Ok(present_modes) = unsafe { surface_loader.get_physical_device_surface_present_modes(gpu, *surface) } {
        if present_modes.is_empty() { continue; }
      } else { continue; }

      if let Ok(gpu_extensions) = unsafe { instance.enumerate_device_extension_properties(gpu) } {
        for extension in extensions {
          let mut found = false;

          for gpu_extension in &gpu_extensions {
            let name = match gpu_extension.extension_name_as_c_str() {
              Ok(name)    => name,
              Err(error)  => panic!("failed to get device extension name with error: {error}")
            };

            if name == unsafe { CStr::from_ptr(*extension) } {
              found = true;
              break;
            }
          }

          if !found { continue 'gpu_loop; }
        }
      } else { continue; }

      device = Some(gpu);
      queue_families = Some(map);
      device_type = properties.device_type;
    };

    if let None = device {
      panic!("failed to find suitable physical device")
    };

    GPU {
      device: device.unwrap(),
      queue_families: queue_families.unwrap()
    }
  }

  pub fn queue_create_infos(&self) -> Vec<vk::DeviceQueueCreateInfo> {
    let mut create_infos = Vec::<vk::DeviceQueueCreateInfo>::new();

    for queue_family in &self.queue_families.map {
      let create_info = {
        vk::DeviceQueueCreateInfo::default()
          .queue_family_index(queue_family.1.index as u32)
          .queue_priorities(&[1f32])
      };

      create_infos.push(create_info);
    }

    create_infos
  }

  pub fn get_queues(&mut self, device: &ash::Device) {
    for (_, queue_family) in &mut self.queue_families.map {
      queue_family.queue = unsafe { device.get_device_queue(queue_family.index as u32, 0) }
    }
  }

  // checks to see if gpu2 has priority over gpu1.
  fn has_priority(gpu1: vk::PhysicalDeviceType, gpu2: vk::PhysicalDeviceType) -> bool {
    match ( gpu1, gpu2 ) {
      ( vk::PhysicalDeviceType::OTHER, _ )                                              => return true,
      ( vk::PhysicalDeviceType::INTEGRATED_GPU, vk::PhysicalDeviceType::DISCRETE_GPU )  => return true,
      ( vk::PhysicalDeviceType::VIRTUAL_GPU, vk::PhysicalDeviceType::DISCRETE_GPU )     => return true,
      ( vk::PhysicalDeviceType::VIRTUAL_GPU, vk::PhysicalDeviceType::INTEGRATED_GPU )   => return true,
      _ => return false
    }
  }
}