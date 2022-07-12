// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Pose__init(msg: *mut Pose) -> bool;
    fn geometry_msgs__msg__Pose__fini(msg: *mut Pose);
    fn geometry_msgs__msg__Pose__Sequence__init(msg: *mut PoseSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__Pose__Sequence__fini(msg: *mut PoseSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Pose() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

impl Pose {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Pose__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Pose {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Pose__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct PoseSeqRaw {
    data: *mut Pose,
    size: usize,
    capacity: usize,
}

/// Sequence of Pose.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PoseSeq<const N: usize> {
    data: *mut Pose,
    size: usize,
    capacity: usize,
}

impl<const N: usize> PoseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PoseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Pose__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Pose]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Pose]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for PoseSeq<N> {
    fn drop(&mut self) {
        let mut msg = PoseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__Pose__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PoseSeq<N> {}
unsafe impl<const N: usize> Sync for PoseSeq<N> {}


impl TopicMsg for Pose {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Pose()
        }
    }
}
