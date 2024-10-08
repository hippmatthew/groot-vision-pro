use sdl2::{event::Event, keyboard::Keycode, video::Window as SDLWindow, Sdl};
use ash::vk::{self, Handle};

use std::vec::Vec;

pub struct Window {
  context: Sdl,
  window: SDLWindow
}

impl Window {
  pub fn new() -> Self {
    let context = match sdl2::init() {
      Ok(context) => context,
      Err(error)  => panic!("failed to initialize SDL with error: {error}")
    };

    let video = match context.video() {
      Ok(video)   => video,
      Err(error)  => panic!("failed to initialize SDL video subsystem with error: {error}")
    };

    let window = match {
      video.window("Groot Vision Pro", 1280, 720)
        .vulkan()
        .fullscreen()
        .build()
    } {
      Ok(window) => window,
      Err(error) => panic!("failed to create SDL window with error: {error}")
    };

    Window {
      context,
      window
    }
  }

  pub fn extensions(&self) -> Vec<*const i8> {
    let str_extensions = match self.window.vulkan_instance_extensions() {
      Ok(ext)     => ext,
      Err(error)  => panic!("failed to get required sdl window extensions with error: {error}")
    };

    let mut extensions = Vec::<*const i8>::new();

    for extension in str_extensions {
      extensions.push(extension.as_ptr() as *const i8);
    }

    extensions
  }

  pub fn surface(&self, instance: &ash::Instance) -> vk::SurfaceKHR {
    match self.window.vulkan_create_surface(instance.handle().as_raw() as usize) {
      Ok(surface) => vk::SurfaceKHR::from_raw(surface),
      Err(error)  => panic!("failed to create window surface with error: {error}")
    }
  }

  pub fn poll_events(&self) -> bool {
    let mut event_pump = match self.context.event_pump() {
      Ok(event_pump)  => event_pump,
      Err(error)      => panic!("failed to get event pump with error: {error}")
    };

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }                                    => return true,
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return true,
        _ => ()
      }
    }

    false
  }
}