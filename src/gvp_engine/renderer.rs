use crate::gvp_engine::gpu::GPU;

use ash::{vk, khr::{surface, swapchain}};

pub struct Renderer {
  frame_index: usize,
  format: vk::SurfaceFormatKHR,
  present_mode: vk::PresentModeKHR,
  extent: vk::Extent2D,
  swapchain_loader: swapchain::Device,
  swapchain: vk::SwapchainKHR,
  images: Vec<vk::Image>,
  image_views: Vec<vk::ImageView>
  // depth_memory: vk::DeviceMemory,
  // depth_buffer: vk::Buffer,
  // depth_image: vk::Image,
  // depth_image_view: vk::ImageView,
  // flight_fences: Vec<vk::Fence>,
  // image_semaphores: Vec<vk::Semaphore>,
  // render_semaphores: Vec<vk::Semaphore>
}

impl Renderer {
  const MAX_FRAME_COUNT: usize = 2;

  pub fn new(
    instance: &ash::Instance,
    device: &ash::Device,
    surface_loader: &surface::Instance,
    surface: &vk::SurfaceKHR,
    gpu: &GPU
  ) -> Self {
    let swapchain_loader = swapchain::Device::new(instance, device);

    let (format, present_mode, extent, image_count, transform) = {
      Renderer::get_swapchain_details(surface_loader, surface, gpu)
    };

    let swapchain = {
      Renderer::create_swapchain(&swapchain_loader, surface, &format, &present_mode, &extent, &image_count, transform)
    };

    let images = match unsafe { swapchain_loader.get_swapchain_images(swapchain) } {
      Ok(images) => images,
      Err(error) => panic!("failed to get swapchain images with error: {error}")
    };

    let image_views = Renderer::get_image_views(device, &images, &format.format);

    Renderer {
      frame_index: 0,
      format,
      present_mode,
      extent,
      swapchain_loader,
      swapchain,
      images,
      image_views
    }
  }

  pub fn clean(&mut self, device: &ash::Device) {
    unsafe {
      for image_view in &self.image_views {
        device.destroy_image_view(*image_view, None);
      }
      self.swapchain_loader.destroy_swapchain(self.swapchain, None)
    };
  }

  fn get_swapchain_details(
    surface_loader: &surface::Instance,
    surface: &vk::SurfaceKHR,
    gpu: &GPU
  ) -> (vk::SurfaceFormatKHR, vk::PresentModeKHR, vk::Extent2D, u32, vk::SurfaceTransformFlagsKHR) {
    let formats = match unsafe { surface_loader.get_physical_device_surface_formats(gpu.device, *surface) } {
      Ok(formats) => formats,
      Err(error)  => panic!("failed to get surface formats with error: {error}")
    };

    // use a format that matches one of the desired formats in the match statement
    // if there are no formats that match, it will default to the first available format
    // ** make a way to output the resulting format if none were matched **
    let format = *formats.iter()
      .find(|format| match (format.format, format.color_space) {
        (vk::Format::R8G8B8A8_SRGB, vk::ColorSpaceKHR::SRGB_NONLINEAR) => true,
        _ => false
      })
      .unwrap_or(&formats[0]);

    let present_modes = match unsafe { surface_loader.get_physical_device_surface_present_modes(gpu.device, *surface) } {
      Ok(present_modes) => present_modes,
      Err(error)        => panic!("failed to get surface present modes with error: {error}")
    };

    // same structure as formats. put desired present modes in the match statement
    // Will default to FIFO is none are matched because FIFO should be available on every surface
    let present_mode = present_modes.iter().cloned()
      .find(|&present_mode| match present_mode {
        vk::PresentModeKHR::MAILBOX => true,
        _ => false
      })
      .unwrap_or(vk::PresentModeKHR::FIFO);

    let capabilities = match unsafe { surface_loader.get_physical_device_surface_capabilities(gpu.device, *surface) } {
      Ok(capabilities)  => capabilities,
      Err(error)        => panic!("failed to get surface capabilities with error: {error}")
    };

    let extent = {
      Renderer::clamp_extent(&capabilities.current_extent, &capabilities.min_image_extent, &capabilities.max_image_extent)
    };

    let mut image_count = capabilities.min_image_count + 1;
    if capabilities.max_image_count > 0 && image_count > capabilities.max_image_count {
      image_count = capabilities.max_image_count;
    }

    (format, present_mode, extent, image_count, capabilities.current_transform)
  }

  fn clamp_extent(extent: &vk::Extent2D, min_extent: &vk::Extent2D, max_extent: &vk::Extent2D) -> vk::Extent2D {
    let mut width = extent.width;
    let mut height = extent.height;

    if width < min_extent.width {
      width = min_extent.width;
    }
    else if width > max_extent.width {
      width = max_extent.width;
    }

    if height < min_extent.height {
      height = min_extent.height;
    }
    else if height > max_extent.height {
      height = max_extent.height;
    }

    vk::Extent2D {
      width,
      height
    }
  }

  fn create_swapchain(
    swapchain_loader: &swapchain::Device,
    surface: &vk::SurfaceKHR,
    format: &vk::SurfaceFormatKHR,
    present_mode: &vk::PresentModeKHR,
    extent: &vk::Extent2D,
    image_count: &u32,
    transform: vk::SurfaceTransformFlagsKHR
  ) -> vk::SwapchainKHR {
    let create_info = {
      vk::SwapchainCreateInfoKHR::default()
        .surface(*surface)
        .min_image_count(*image_count)
        .image_format(format.format)
        .image_color_space(format.color_space)
        .present_mode(*present_mode)
        .image_extent(*extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .pre_transform(transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .clipped(true)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
    };

    match unsafe { swapchain_loader.create_swapchain(&create_info, None) } {
      Ok(swapchain) => swapchain,
      Err(error)    => panic!("failed to create swapchain with error: {error}")
    }
  }

  fn get_image_views(device: &ash::Device, images: &Vec<vk::Image>, format: &vk::Format) -> Vec<vk::ImageView> {
    let component_mapping = {
      vk::ComponentMapping::default()
        .r(vk::ComponentSwizzle::IDENTITY)
        .g(vk::ComponentSwizzle::IDENTITY)
        .g(vk::ComponentSwizzle::IDENTITY)
        .a(vk::ComponentSwizzle::IDENTITY)
    };

    let subresource_range = {
      vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
    };

    let mut image_views = Vec::<vk::ImageView>::new();

    for image in images {
      let create_info = {
        vk::ImageViewCreateInfo::default()
          .image(*image)
          .view_type(vk::ImageViewType::TYPE_2D)
          .format(*format)
          .components(component_mapping)
          .subresource_range(subresource_range)
      };

      let image_view = match unsafe { device.create_image_view(&create_info, None) } {
        Ok(image_view)  => image_view,
        Err(error)      => panic!("failed to create image view with error: {error}")
      };

      image_views.push(image_view);
    }

    image_views
  }
}