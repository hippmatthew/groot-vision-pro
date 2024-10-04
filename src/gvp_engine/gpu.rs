use ash::{vk, khr::surface};

use std::{collections::HashMap};

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

pub struct GPU {
  device: vk::PhysicalDevice,
  queue_families: HashMap<QueueFamilyType, QueueFamily>
}

impl GPU {
  // Needed methods:
  // 1. get method to automatically acquire gpu
  // 2. methods for accessing queue families
  // 3. query whether the gpu contains a specific queue

  pub fn get(instance: &vk::Instance, surface_loader: &surface::Instance, surface: &vk::SurfaceKHR) -> Self {
    // 1. get a list of all gpus
    // 2. loop through all gpus and check if they are suitable
    //      - Must have a main queue
    //      - Must have valid surface formats / present modes
    //      - Must support all required extensions
    // 3. Choose GPU off of priority:
    //      - Physical GPU
    //      - Integrated
    //      - Virtual
    // 4. Store gpu in a variable called best option. Move most suitable gpu into it based on priority
    // 5. Store the queue families


  }
}