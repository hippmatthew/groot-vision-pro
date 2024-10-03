use super::gui::GUI;
use ash::{vk, khr};
use sdl2::{event::Event, keyboard::Keycode};
use std::{ffi::CStr, vec::Vec};

macro_rules! c_str {
  ($s:expr) => {
    unsafe { CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
  };
}

macro_rules! gvp_version {
  () => {
    vk::make_api_version(0, 0, 2, 0)
  }
}

pub struct GVPengine {
  gui: GUI,
  #[allow(dead_code)]
  entry: ash::Entry,
  vk_instance: ash::Instance
}

impl GVPengine {
  pub fn init() -> Self {
    let gui = GUI::new();
    let (entry, vk_instance) = GVPengine::create_instance();

    GVPengine {
      gui,
      entry,
      vk_instance
    }
  }

  pub fn run(&self) {
    'main_loop: loop {
      for event in self.gui.sdl_context.event_pump().unwrap().poll_iter() {
        match event {
          Event::Quit {..} => break 'main_loop,
          Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
          _ => ()
        }
      }
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

impl Drop for GVPengine {
  fn drop(&mut self) {
    unsafe {
      self.vk_instance.destroy_instance(None);
    }
  }
}