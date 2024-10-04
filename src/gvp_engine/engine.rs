use ash::vk;

use std::{ffi::CStr, vec::Vec};

macro_rules! c_str {
  ($s:expr) => {
    unsafe { CStr::from_bytes_with_nul_unchecked(format!("{}\0", $s).as_bytes()) }
  };
}

macro_rules! gvp_version {
  () => { vk::make_api_version(0, 0, 3, 0) };
}

pub struct GVPEngine {
  entry: ash::Entry,
  instance: ash::Instance
}

impl GVPEngine {
  pub fn init() -> Self {
    let entry = match unsafe { ash::Entry::load() } {
      Ok(entry) => entry,
      Err(error) => panic!("failed to load vulkan with error: {error}")
    };

    let instance = GVPEngine::create_instance(&entry);

    GVPEngine {
      entry,
      instance
    }
  }

  fn create_instance(entry: &ash::Entry) -> ash::Instance {
    let application_info = {
      vk::ApplicationInfo::default()
        .application_name(c_str!("Groot Vision Pro"))
        .application_version(gvp_version!())
        .engine_name(c_str!("GVP Engine"))
        .engine_version(gvp_version!())
        .api_version(vk::API_VERSION_1_3)
    };

    let layers = [ c_str!("VK_KHR_validation").as_ptr() ];
    let extensions = Vec::<*const i8>::new();

    let properties = match unsafe { entry.enumerate_instance_extension_properties(None) } {
      Ok(properties) => properties,
      Err(error) => panic!("failed to get instance extension properties with error: {error}")
    };

    for property in properties {
      if property.extension_name. {
        continue;
      }
    };
  }
}