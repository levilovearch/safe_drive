// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__QuaternionStamped__init(msg: *mut QuaternionStamped) -> bool;
    fn geometry_msgs__msg__QuaternionStamped__fini(msg: *mut QuaternionStamped);
    fn geometry_msgs__msg__QuaternionStamped__Sequence__init(msg: *mut QuaternionStampedSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__QuaternionStamped__Sequence__fini(msg: *mut QuaternionStampedSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__QuaternionStamped() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct QuaternionStamped {
    pub header: std_msgs::msg::Header,
    pub quaternion: Quaternion,
}

impl QuaternionStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__QuaternionStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for QuaternionStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__QuaternionStamped__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct QuaternionStampedSeqRaw {
    data: *mut QuaternionStamped,
    size: usize,
    capacity: usize,
}

/// Sequence of QuaternionStamped.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct QuaternionStampedSeq<const N: usize> {
    data: *mut QuaternionStamped,
    size: usize,
    capacity: usize,
}

impl<const N: usize> QuaternionStampedSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: QuaternionStampedSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__QuaternionStamped__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[QuaternionStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [QuaternionStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for QuaternionStampedSeq<N> {
    fn drop(&mut self) {
        let mut msg = QuaternionStampedSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__QuaternionStamped__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for QuaternionStampedSeq<N> {}
unsafe impl<const N: usize> Sync for QuaternionStampedSeq<N> {}


impl TopicMsg for QuaternionStamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__QuaternionStamped()
        }
    }
}
