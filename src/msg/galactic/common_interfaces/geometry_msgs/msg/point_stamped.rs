// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__PointStamped__init(msg: *mut PointStamped) -> bool;
    fn geometry_msgs__msg__PointStamped__fini(msg: *mut PointStamped);
    fn geometry_msgs__msg__PointStamped__Sequence__init(msg: *mut PointStampedSequence, size: usize) -> bool;
    fn geometry_msgs__msg__PointStamped__Sequence__fini(msg: *mut PointStampedSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PointStamped() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct PointStamped {
    pub header: std_msgs::msg::Header,
    pub point: Point,
}

impl PointStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__PointStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for PointStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__PointStamped__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PointStampedSequence {
    data: *mut PointStamped,
    size: usize,
    capacity: usize,
}

impl PointStampedSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__PointStamped__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[PointStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [PointStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for PointStampedSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__PointStamped__Sequence__fini(self) };
    }
}

impl TopicMsg for PointStamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PointStamped()
        }
    }
}
