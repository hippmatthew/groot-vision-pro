use super::gui::GUI;
use ash::{khr::surface, vk};
use sdl2::{event::Event, keyboard::Keycode};
use std::ffi::CStr;

macro_rules! c_str {
    ($s:expr) => {
        unsafe { CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
    };
}

macro_rules! gvp_version {
    () => {
        vk::make_api_version(0, 0, 3, 0)
    };
}

pub struct GVPengine {
    gui: GUI,
    #[allow(dead_code)]
    entry: ash::Entry,
    vk_instance: ash::Instance,
    surface_loader: surface::Instance,
}

impl GVPengine {
    pub fn init() -> Self {
        let mut gui = GUI::new();

        let (entry, vk_instance) = GVPengine::create_instance(&gui);
        let surface_loader = surface::Instance::new(&entry, &vk_instance);

        gui.create_surface(&vk_instance);

        GVPengine {
            gui,
            entry,
            vk_instance,
            surface_loader,
        }
    }

    pub fn poll_events(&self) -> bool {
        for event in self.gui.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => (),
            }
        }

        false
    }

    fn create_instance(gui: &GUI) -> (ash::Entry, ash::Instance) {
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
            Err(error) => {
                panic!("failed to enumerate instance extension properties with error {error}")
            }
        };

        let mut create_flags = vk::InstanceCreateFlags::default();
        let mut instance_extensions = gui.extensions();
        let layer_names = [c_str!("VK_LAYER_KHRONOS_validation").as_ptr()];

        for property in properties {
            if property.extension_name_as_c_str().unwrap() != vk::KHR_PORTABILITY_ENUMERATION_NAME {
                continue;
            }

            create_flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
            instance_extensions.push(vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr());
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
            self.surface_loader
                .destroy_surface(self.gui.vk_surface, None);
            self.vk_instance.destroy_instance(None);
        }
    }
}
