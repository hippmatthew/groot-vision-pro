use ash::{vk, khr::surface};

use std::collections::HashMap;

pub enum QueueFamilyType {
  Main,
  Async,
  Compute,
  Transfer,
  Sparse
}

pub struct QueueFamily {
  pub index: usize,
  pub queue: vk::Queue
}

pub struct QueueFamilyMap {
  map: HashMap<QueueFamilyType, QueueFamily>
}

impl QueueFamily {
  pub fn new(index: usize) -> Self {
    QueueFamily {
      index,
      queue: vk::Queue::default()
    }
  }

  pub fn type(
    instance: &ash::Instance,
    surface_loader: &surface::Instance,
    surface: &vk::SurfaceKHR
  ) -> QueueFamilyType {

  }
}

struct MissingQueueFamily;

impl QueueFamilyMap {
  pub fn new(instance: &ash::Instance, surface_loader: &surface::Instance, surface: &vk::SurfaceKHR) -> Self {

  }

  pub fn contains(&self, family_type: QueueFamilyType) -> bool {

  }
}