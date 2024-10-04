use sdl2::{Sdl, VideoSubsystem, video::Window as SDLWindow};

use std::vec::Vec;

pub struct Window {
  context: Sdl,
  video: VideoSubsystem,
  window: SDLWindow
}

impl Window {
  pub fn new() -> Self {
    let context = match sdl2::init() {
      Ok(context) => context,
      Err(error) => panic!("failed to initialize SDL with error: {error}")
    };

    let video = match context.video() {
      Ok(video) => video,
      Err(error) => panic!("failed to initialize SDL video subsystem with error: {error}")
    };

    let window =

    Window {
      context,
      video
    }
  }

  pub fn extensions() -> Vec<*const i8> {
    Vec::<*const i8>::new();
  }
}