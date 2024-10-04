use ash::vk;

macro_rules! c_str() {
  ($s:expr) => {

  }
}

pub struct GVPEngine {
  entry: ash::Entry,
  instance: ash::Instance
}

impl GVPEngine {
  pub fn init() -> Self {
    let (entry, instance) = GVPEngine::create_instance();

    GVPEngine {
      entry,
      instance
    }
  }

  fn create_instance() -> (ash::Entry, ash::Instance) {
    let application_info = {
      vk::ApplicationInfo::default()
    };
  }
}