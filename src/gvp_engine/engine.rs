use crate::gvp_engine::{window::Window, gpu::GPU, renderer::Renderer};

use ash::{vk, khr::surface};

use std::ffi::CStr;

macro_rules! c_str {
  ($s:expr) => {
    unsafe { CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
  };
}

macro_rules! gvp_version {
  () => { vk::make_api_version(0, 0, 6, 0) };
}

pub struct GVPEngine {
  window: Window,
  gpu: GPU,
  instance: ash::Instance,
  surface_loader: surface::Instance,
  surface: vk::SurfaceKHR,
  device: ash::Device,
  renderer: Renderer
}

impl GVPEngine {
  pub fn init() -> Self {
    let entry = match unsafe { ash::Entry::load() } {
      Ok(entry) => entry,
      Err(error) => panic!("failed to load vulkan with error: {error}")
    };

    let window = Window::new();
    let instance = GVPEngine::create_instance(&window, &entry);
    let surface_loader = surface::Instance::new(&entry, &instance);
    let surface = window.surface(&instance);

    let mut required_extensions = vec![
      vk::KHR_SWAPCHAIN_NAME.as_ptr(),
      vk::KHR_DYNAMIC_RENDERING_NAME.as_ptr()
    ];

    let mut gpu = GPU::get(&instance, &surface_loader, &surface, &required_extensions);
    let device = GVPEngine::create_device(&instance, &gpu, &mut required_extensions);

    gpu.get_queues(&device);

    let renderer = Renderer::new(&instance, &device, &surface_loader, &surface, &gpu);

    GVPEngine {
      window,
      instance,
      surface_loader,
      surface,
      gpu,
      device,
      renderer
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
      Ok(properties)  => properties,
      Err(error)      => panic!("failed to get instance extension properties with error: {error}")
    };

    for property in properties {
      let name = match property.extension_name_as_c_str() {
        Ok(name)    => name,
        Err(error)  => panic!("failed to get extension name with error: {error}")
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
      Ok(instance)  => instance,
      Err(error)    => panic!("failed to create instance with error: {error}")
    }
  }

  fn create_device(
    instance: &ash::Instance,
    gpu: &GPU,
    required_extensions: &mut Vec<*const i8>
  ) -> ash::Device {
    let device_extensions = match unsafe { instance.enumerate_device_extension_properties(gpu.device) } {
      Ok(extensions)  => extensions,
      Err(error)      => panic!("failed to get gpu extensions to check for portability subset with error: {error}")
    };

    for extension in device_extensions {
      let name = match extension.extension_name_as_c_str() {
        Ok(name)    => name,
        Err(error)  => panic!("failed to get extension name for device creation with error {error}")
      };

      if name != vk::KHR_PORTABILITY_SUBSET_NAME { continue; }

      required_extensions.push(vk::KHR_PORTABILITY_SUBSET_NAME.as_ptr());
      break;
    }

    let queue_create_infos = gpu.queue_create_infos();
    let features = vk::PhysicalDeviceFeatures::default();
    let mut dynamic_rendering = {
      vk::PhysicalDeviceDynamicRenderingFeatures::default()
        .dynamic_rendering(true)
    };

    let create_info = {
      vk::DeviceCreateInfo::default()
        .enabled_extension_names(&required_extensions)
        .enabled_features(&features)
        .queue_create_infos(&queue_create_infos)
        .push_next(&mut dynamic_rendering)
    };

    match unsafe { instance.create_device(gpu.device, &create_info, None) } {
      Ok(device) => device,
      Err(error) => panic!("failed to create device with error: {error}")
    }
  }
}

impl Drop for GVPEngine {
  fn drop(&mut self) {
    unsafe{
      self.renderer.clean(&self.device);
      self.device.destroy_device(None);
      self.surface_loader.destroy_surface(self.surface, None);
      self.instance.destroy_instance(None);
    }
  }
}