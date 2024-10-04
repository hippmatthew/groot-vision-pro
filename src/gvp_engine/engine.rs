use crate::gvp_engine::window::Window;

use ash::{vk, khr::surface};

use std::{ffi::CStr, vec::Vec};

macro_rules! c_str {
  ($s:expr) => {
    unsafe { CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
  };
}

macro_rules! gvp_version {
  () => { vk::make_api_version(0, 0, 3, 0) };
}

pub struct GVPEngine {
  window: Window,
  entry: ash::Entry,
  instance: ash::Instance,
}

impl GVPEngine {
  pub fn init() -> Self {
    let window = Window::new();

    let entry = match unsafe { ash::Entry::load() } {
      Ok(entry) => entry,
      Err(error) => panic!("failed to load vulkan with error: {error}")
    };

    let instance = GVPEngine::create_instance(&window, &entry);

    GVPEngine {
      window,
      entry,
      instance,
    }
  }

  pub fn poll_events(&self) -> bool {
    self.window.poll_events()
  }

  fn create_instance(window: &Window, entry: &ash::Entry) -> ash::Instance {
    let application_info = {
      vk::ApplicationInfo::default()
        .application_name(c_str!("Groot Vision Pro"))
        .application_version(gvp_version!())
        .engine_name(c_str!("GVP Engine"))
        .engine_version(gvp_version!())
        .api_version(vk::API_VERSION_1_3)
    };

    let layers = [ c_str!("VK_LAYER_KHRONOS_validation").as_ptr() ];
    let mut extensions = window.extensions();
    let mut flags = vk::InstanceCreateFlags::default();

    let properties = match unsafe { entry.enumerate_instance_extension_properties(None) } {
      Ok(properties) => properties,
      Err(error) => panic!("failed to get instance extension properties with error: {error}")
    };

    for property in properties {
      let name = match property.extension_name_as_c_str() {
        Ok(name) => name,
        Err(error) => panic!("failed to get extension name with error: {error}")
      };

      if name != vk::KHR_PORTABILITY_ENUMERATION_NAME {
        continue;
      }

      extensions.push(vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr());
      extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_NAME.as_ptr());
      flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;

      break;
    };

    let create_info = {
      vk::InstanceCreateInfo::default()
        .flags(flags)
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
    };

    match unsafe { entry.create_instance(&create_info, None) } {
      Ok(instance) => instance,
      Err(error) => panic!("failed to create instance with error: {error}")
    }
  }
}

impl Drop for GVPEngine {
  fn drop(&mut self) {
    unsafe{ self.instance.destroy_instance(None) };
  }
}