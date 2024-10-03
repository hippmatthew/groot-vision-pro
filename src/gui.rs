use sdl2::{Sdl, VideoSubsystem, video::Window};

pub struct GUI {
  pub sdl_context: Sdl,
  #[allow(dead_code)]
  pub sdl_video: VideoSubsystem,
  #[allow(dead_code)]
  pub sdl_window: Window
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

    let window_result = sdl_video.window("Groot Vision Pro", 720, 1280)
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
    }
  }
}