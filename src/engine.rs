use ash::{vk, khr};
use std::{ffi::CStr, vec::Vec};

macro_rules! c_str {
  ($s:expr) => {
    unsafe { CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
  };
}

macro_rules! gvp_version {
  () => {
    vk::make_api_version(0, 0, 1, 0)
  }
}

pub struct GVPengine {
  entry : ash::Entry,
  vk_instance : ash::Instance
}

impl GVPengine {
  pub fn new() -> Self {
    let (entry, vk_instance) = GVPengine::create_instance();

    GVPengine {
      entry,
      vk_instance
    }
  }

  fn create_instance() -> (ash::Entry, ash::Instance) {
    let entry = match unsafe { ash::Entry::load() } {
      Ok(e) => e,
      Err(error) => panic!("Failed to find vulkan entry point with error {error}"),
    };

    let app_info = vk::ApplicationInfo::default()
      .application_name(c_str!("Groot Vision Pro"))
      .application_version(gvp_version!())
      .engine_name(c_str!("GVP_Engine"))
      .engine_version(gvp_version!())
      .api_version(vk::make_api_version(0, 1, 3, 0));

    let properties = match unsafe { entry.enumerate_instance_extension_properties(None) } {
      Ok(p) => p,
      Err(error) => panic!("failed to enumerate instance extension properties with error {error}"),
    };

    let mut create_flags = vk::InstanceCreateFlags::default();
    let mut instance_extensions = Vec::<*const i8>::new();
    let layer_names = [c_str!("VK_LAYER_KHRONOS_validation").as_ptr()];

    for property in properties {
      if property.extension_name_as_c_str().unwrap() != vk::KHR_PORTABILITY_ENUMERATION_NAME { continue; }

      create_flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
      instance_extensions.push(khr::portability_enumeration::NAME.as_ptr());
      instance_extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_NAME.as_ptr());

      break;
    }

    let ci_instance = vk::InstanceCreateInfo::default()
      .application_info(&app_info)
      .enabled_layer_names(&layer_names)
      .enabled_extension_names(&instance_extensions)
      .flags(create_flags);

    let vk_instance = match unsafe { entry.create_instance(&ci_instance, None) } {
      Ok(i) => i,
      Err(error) => panic!("failed to create vulkan instance with error: {error}"),
    };

    (entry, vk_instance)
  }
}