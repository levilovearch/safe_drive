// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn unique_identifier_msgs__msg__UUID__init(msg: *mut UUID) -> bool;
    fn unique_identifier_msgs__msg__UUID__fini(msg: *mut UUID);
    fn unique_identifier_msgs__msg__UUID__Sequence__init(msg: *mut UUIDSeqRaw, size: usize) -> bool;
    fn unique_identifier_msgs__msg__UUID__Sequence__fini(msg: *mut UUIDSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__unique_identifier_msgs__msg__UUID() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct UUID {
    pub uuid: [u8; 16],
}

impl UUID {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { unique_identifier_msgs__msg__UUID__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for UUID {
    fn drop(&mut self) {
        unsafe { unique_identifier_msgs__msg__UUID__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct UUIDSeqRaw {
    data: *mut UUID,
    size: usize,
    capacity: usize,
}

/// Sequence of UUID.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct UUIDSeq<const N: usize> {
    data: *mut UUID,
    size: usize,
    capacity: usize,
}

impl<const N: usize> UUIDSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: UUIDSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { unique_identifier_msgs__msg__UUID__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[UUID]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [UUID]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for UUIDSeq<N> {
    fn drop(&mut self) {
        let mut msg = UUIDSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { unique_identifier_msgs__msg__UUID__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for UUIDSeq<N> {}
unsafe impl<const N: usize> Sync for UUIDSeq<N> {}


impl TopicMsg for UUID {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__unique_identifier_msgs__msg__UUID()
        }
    }
}
