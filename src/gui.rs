use ash::vk::{self, Handle};
use sdl2::{video::Window, Sdl, VideoSubsystem};
use std::{ffi::CStr, vec::Vec};

pub struct GUI {
    pub sdl_context: Sdl,
    #[allow(dead_code)]
    pub sdl_video: VideoSubsystem,
    #[allow(dead_code)]
    pub sdl_window: Window,
    #[allow(dead_code)]
    pub vk_surface: vk::SurfaceKHR,
}

impl GUI {
    pub fn new() -> Self {
        let sdl_context = match sdl2::init() {
            Ok(c) => c,
            Err(error) => panic!("failed to initialize sdl2 with error: {error}"),
        };

        let sdl_video = match sdl_context.video() {
            Ok(v) => v,
            Err(error) => panic!("failed to get the SDL2 video subsystem with error: {error}"),
        };

        let window_result = sdl_video
            .window("Groot Vision Pro", 720, 1280)
            .vulkan()
            .fullscreen()
            .build();

        let sdl_window = match window_result {
            Ok(w) => w,
            Err(error) => panic!("failed to create window with error: {error}"),
        };

        GUI {
            sdl_context,
            sdl_video,
            sdl_window,
            vk_surface: vk::SurfaceKHR::default(),
        }
    }

    pub fn extensions(&self) -> Vec<*const i8> {
        let ext = match self.sdl_window.vulkan_instance_extensions() {
            Ok(e) => e,
            Err(error) => panic!("failed to get instance extensions with error: {error}"),
        };

        let mut extensions = Vec::<*const i8>::new();

        for extension in ext {
            extensions
                .push(unsafe { CStr::from_bytes_with_nul_unchecked(extension.as_bytes()).as_ptr() })
        }

        extensions
    }

    pub fn create_surface(&mut self, vk_instance: &ash::Instance) {
        self.vk_surface = vk::SurfaceKHR::from_raw(
            self.sdl_window
                .vulkan_create_surface(vk_instance.handle().as_raw() as usize)
                .unwrap(),
        );
    }
}
