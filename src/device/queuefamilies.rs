use ash::{khr, vk};
use std::{
    collections::{HashMap, VecDeque},
    ops::{BitAnd, BitOr, BitOrAssign},
};

enum QueueType {
    Graphics = 0b1000000,
    Present = 0b0100000,
    SyncCompute = 0b0010000,
    SyncTransfer = 0b0001000,
    AsyncCompute = 0b0000100,
    AsyncTransfer = 0b0000010,
    Sparse = 0b0000001,
}

pub enum QueueFamilyType {
    All,
    Async,
    Compute,
    Transfer,
    Sparse,
}

pub struct QueueFamily {
    index: usize,
    vk_queue: vk::Queue,
}

pub struct QueueFamilies {
    families: HashMap<String, QueueFamily>,
}

impl BitOr for QueueType {
    type Output = u8;

    fn bitor(self, other: Self) -> Self::Output {
        self as u8 | other as u8
    }
}

impl BitAnd for QueueType {
    type Output = u8;

    fn bitand(self, other: Self) -> Self::Output {
        self as u8 & other as u8
    }
}

impl BitOr<u8> for QueueType {
    type Output = u8;

    fn bitor(self, other: u8) -> Self::Output {
        self as u8 | other
    }
}

impl BitAnd<u8> for QueueType {
    type Output = u8;

    fn bitand(self, other: u8) -> Self::Output {
        self as u8 & other
    }
}

impl BitOr<QueueType> for u8 {
    type Output = u8;

    fn bitor(self, other: QueueType) -> Self::Output {
        self | other as u8
    }
}

impl BitAnd<QueueType> for u8 {
    type Output = u8;

    fn bitand(self, other: QueueType) -> Self::Output {
        self & other as u8
    }
}

impl QueueFamily {
    pub fn new(index: usize) -> Self {
        QueueFamily {
            index,
            vk_queue: vk::Queue::default(),
        }
    }
}

impl QueueFamilies {
    pub fn get(
        vk_instance: &ash::Instance,
        surface_loader: &khr::surface::Instance,
        vk_surface: &vk::SurfaceKHR,
        vk_physical_device: &vk::PhysicalDevice,
    ) -> Self {
        let mut all = VecDeque::<usize>::new();
        let mut asynch = VecDeque::<usize>::new();
        let mut compute = VecDeque::<usize>::new();
        let mut transfer = VecDeque::<usize>::new();
        let mut sparse = VecDeque::<usize>::new();

        let properties =
            unsafe { vk_instance.get_physical_device_queue_family_properties(*vk_physical_device) };

        let mut index: usize = 0;
        for property in properties {
            match QueueFamilies::queue_family_type(
                surface_loader,
                vk_surface,
                vk_physical_device,
                property,
                index,
            ) {
                QueueFamilyType::All => all.push_back(index),
                QueueFamilyType::Async => asynch.push_back(index),
                QueueFamilyType::Compute => compute.push_back(index),
                QueueFamilyType::Transfer => transfer.push_back(index),
                QueueFamilyType::Sparse => sparse.push_back(index),
            }

            index += 1;
        }

        // Rules for choosing queue families:
        // 1. There should always be 1 All queue
        // 2. Prefer to have 1 All, 1 Compute, and 1 Transfer queue
        // 3. If there are not dedicated compute/transfer queues but there are 2 async queues, use 1 async queue for
        //    async compute operations and the other for async transfer operations
        // 4. If there are not enough async queues, but 2 or more all queues (after the dedicated all queue), use 1 all
        //    queue for async compute operations and the other for async transfer operations
        // 5. If there are not enough async or all queues to have separate async compute/transfer queues, check for an
        //    async queue. If there is an async queue, use that for all async compute/transfer operations
        // 6.If there is not an asynch queue but an extra all queue, use the all queue for asynch compute/transfer
        //   operations
        // 7. Sparse queues are optional

        let mut families = HashMap::<String, QueueFamily>::new();

        if !all.is_empty() {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::All),
                QueueFamily::new(*all.front().unwrap()),
            );
            all.pop_front();
        }

        if !compute.is_empty() && !transfer.is_empty() {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Compute),
                QueueFamily::new(*compute.front().unwrap()),
            );
            compute.pop_front();

            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Transfer),
                QueueFamily::new(*transfer.front().unwrap()),
            );
            transfer.pop_front();
        } else if asynch.len() >= 2 {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Compute),
                QueueFamily::new(*asynch.front().unwrap()),
            );
            asynch.pop_front();

            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Transfer),
                QueueFamily::new(*asynch.front().unwrap()),
            );
            asynch.pop_front();
        } else if all.len() >= 2 {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Compute),
                QueueFamily::new(*all.front().unwrap()),
            );
            all.pop_front();

            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Transfer),
                QueueFamily::new(*all.front().unwrap()),
            );
            all.pop_front();
        } else if !asynch.is_empty() {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Async),
                QueueFamily::new(*asynch.front().unwrap()),
            );
            asynch.pop_front();
        } else if !all.is_empty() {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Async),
                QueueFamily::new(*all.front().unwrap()),
            );
            all.pop_front();
        }

        if !sparse.is_empty() {
            families.insert(
                QueueFamilies::family_to_string(QueueFamilyType::Sparse),
                QueueFamily::new(*sparse.front().unwrap()),
            );
            sparse.pop_front();
        }

        QueueFamilies { families }
    }

    pub fn has_family(&self, family_type: QueueFamilyType) -> bool {
        self.families
            .contains_key(&QueueFamilies::family_to_string(family_type))
    }

    pub fn queue(&self, family_type: QueueFamilyType) -> Option<vk::Queue> {
        match self
            .families
            .get(&QueueFamilies::family_to_string(family_type))
        {
            Some(qf) => Some(qf.vk_queue),
            None => None,
        }
    }

    fn queue_family_type(
        surface_loader: &khr::surface::Instance,
        vk_surface: &vk::SurfaceKHR,
        vk_physical_device: &vk::PhysicalDevice,
        property: vk::QueueFamilyProperties,
        index: usize,
    ) -> QueueFamilyType {
        // automatically checks for sparse queue
        if property.queue_flags & vk::QueueFlags::SPARSE_BINDING == vk::QueueFlags::SPARSE_BINDING {
            return QueueFamilyType::Sparse;
        }

        let surface_support = match unsafe {
            surface_loader.get_physical_device_surface_support(
                *vk_physical_device,
                index as u32,
                *vk_surface,
            )
        } {
            Ok(result) => result,
            Err(error) => {
                panic!("failed to get queue family {index} surface support with error: {error}")
            }
        };

        // all queue check (graphics, present, sync compute, sync transfer)
        let filter = vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;
        if property.queue_flags & filter == filter && surface_support {
            return QueueFamilyType::All;
        }

        // async queue check
        let filter = vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;
        if property.queue_flags & filter == filter {
            return QueueFamilyType::Async;
        }

        // compute queue check
        let filter = vk::QueueFlags::COMPUTE;
        if property.queue_flags & filter == filter {
            return QueueFamilyType::Compute;
        }

        // transfer queue check
        let filter = vk::QueueFlags::TRANSFER;
        if property.queue_flags & filter == filter {
            return QueueFamilyType::Transfer;
        }

        panic!("failed to find queue family type within current range of filters")
    }

    fn family_to_string(family_type: QueueFamilyType) -> String {
        match family_type {
            QueueFamilyType::All => String::from("all"),
            QueueFamilyType::Async => String::from("async"),
            QueueFamilyType::Compute => String::from("compute"),
            QueueFamilyType::Transfer => String::from("transfer"),
            QueueFamilyType::Sparse => String::from("sparse"),
        }
    }
}
