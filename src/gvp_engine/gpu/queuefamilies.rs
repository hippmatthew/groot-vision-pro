use ash::{vk, khr::surface};

use std::{
  collections::{HashMap, VecDeque},
  hash::{Hash, Hasher},
  mem::discriminant,
  str::FromStr
};

pub enum QueueFamilyType {
  Main,
  Async,
  Compute,
  Transfer,
  Sparse
}

impl QueueFamilyType {
  pub fn string(family_type: &Self) -> String {
    match family_type {
      QueueFamilyType::Main     => return String::from_str("main").unwrap(),
      QueueFamilyType::Async    => return String::from_str("async").unwrap(),
      QueueFamilyType::Compute  => return String::from_str("compute").unwrap(),
      QueueFamilyType::Transfer => return String::from_str("transfer").unwrap(),
      QueueFamilyType::Sparse   => return String::from_str("sparse").unwrap()
    }
  }
}

impl Hash for QueueFamilyType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    QueueFamilyType::string(self).hash(state);
  }
}

impl Eq for QueueFamilyType {}

impl PartialEq for QueueFamilyType {
  fn eq(&self, rhs: &Self) -> bool {
    discriminant(self) == discriminant(rhs)
  }
}

pub struct QueueFamily {
  pub index: usize,
  pub queue: vk::Queue
}

impl QueueFamily {
  pub fn new(index: usize) -> Self {
    QueueFamily {
      index,
      queue: vk::Queue::default()
    }
  }

  pub fn find_type(
    surface_loader: &surface::Instance,
    surface: &vk::SurfaceKHR,
    device: &vk::PhysicalDevice,
    queue_flags: vk::QueueFlags,
    index: usize
  ) -> QueueFamilyType {

    let filter = vk::QueueFlags::SPARSE_BINDING;
    if queue_flags & filter == filter {
      return QueueFamilyType::Sparse
    }

    let filter = vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;

    if queue_flags & filter == filter {
      if let Ok(_) = unsafe { surface_loader.get_physical_device_surface_support(*device, index as u32, *surface) } {
        return QueueFamilyType::Main
      }
    }

    let filter = vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;
    if queue_flags & filter == filter {
      return QueueFamilyType::Async
    }

    let filter = vk::QueueFlags::COMPUTE;
    if queue_flags & filter == filter {
      return QueueFamilyType::Compute
    }

    let filter = vk::QueueFlags::TRANSFER;
    if queue_flags & filter == filter {
      return QueueFamilyType::Transfer
    }

    panic!("failed to find queue family type within current range of filters")
  }
}

pub struct QueueFamilyMap {
  map: HashMap<QueueFamilyType, QueueFamily>
}

impl QueueFamilyMap {
  pub fn populate(
    instance: &ash::Instance,
    surface_loader: &surface::Instance,
    surface: &vk::SurfaceKHR,
    device: &vk::PhysicalDevice
  ) -> Self {
    let mut main_queues = VecDeque::<usize>::new();
    let mut async_queues = VecDeque::<usize>::new();
    let mut compute_queues = VecDeque::<usize>::new();
    let mut transfer_queues = VecDeque::<usize>::new();
    let mut sparse_queues = VecDeque::<usize>::new();

    let properties =
      unsafe { instance.get_physical_device_queue_family_properties(*device) };

    let mut index: usize = 0;
    for property in properties {
      match QueueFamily::find_type(
        surface_loader,
        surface,
        device,
        property.queue_flags,
        index,
      ) {
        QueueFamilyType::Main     => main_queues.push_back(index),
        QueueFamilyType::Async    => async_queues.push_back(index),
        QueueFamilyType::Compute  => compute_queues.push_back(index),
        QueueFamilyType::Transfer => transfer_queues.push_back(index),
        QueueFamilyType::Sparse   => sparse_queues.push_back(index),
      }

      index += 1;
    }

    // Rules for choosing queue families:
    // 1. There should always be 1 Main queue
    // 2. Prefer to have 1 Main, 1 Compute, and 1 Transfer queue
    // 3. If there are not dedicated compute/transfer queues but there are 2 async queues, use 1 async queue for
    //    async compute operations and the other for async transfer operations
    // 4. If there are not enough async queues, but 2 or more main queues (after the dedicated main queue), use 1 main
    //    queue for async compute operations and the other for async transfer operations
    // 5. If there are not enough async or main queues to have separate async compute/transfer queues, check for an
    //    async queue. If there is an async queue, use that for main async compute/transfer operations
    // 6. If there is not an asynch queue but an extra main queue, use the main queue for async compute/transfer
    //    operations
    // 7. Sparse queues are optional

    let mut map = HashMap::<QueueFamilyType, QueueFamily>::new();

    if let Some(index) = main_queues.pop_front() {
      map.insert(QueueFamilyType::Main, QueueFamily::new(index));
    }

    if !compute_queues.is_empty() && !transfer_queues.is_empty() {
      map.insert(QueueFamilyType::Compute, QueueFamily::new(compute_queues.pop_front().unwrap()));
      map.insert(QueueFamilyType::Transfer, QueueFamily::new(transfer_queues.pop_front().unwrap()));
    }
    else if async_queues.len() >= 2 {
      map.insert(QueueFamilyType::Compute, QueueFamily::new(async_queues.pop_front().unwrap()));
      map.insert(QueueFamilyType::Transfer, QueueFamily::new(async_queues.pop_front().unwrap()));
    }
    else if main_queues.len() >= 2 {
      map.insert(QueueFamilyType::Compute, QueueFamily::new(main_queues.pop_front().unwrap()));
      map.insert(QueueFamilyType::Transfer, QueueFamily::new(main_queues.pop_front().unwrap()));
    }
    else if !async_queues.is_empty() {
      map.insert(QueueFamilyType::Async, QueueFamily::new(async_queues.pop_front().unwrap()));
    }
    else if !main_queues.is_empty() {
      map.insert(QueueFamilyType::Async, QueueFamily::new(main_queues.pop_front().unwrap()),);
    }

    if let Some(index) = sparse_queues.pop_front() {
      map.insert(QueueFamilyType::Sparse, QueueFamily::new(index));
    }

    QueueFamilyMap { map }
  }

  pub fn contains(&self, family_type: &QueueFamilyType) -> bool {
    self.map.contains_key(family_type)
  }
}