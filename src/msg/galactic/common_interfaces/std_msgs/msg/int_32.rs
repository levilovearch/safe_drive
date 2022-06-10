// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Int32__init(msg: *mut Int32) -> bool;
    fn std_msgs__msg__Int32__fini(msg: *mut Int32);
    fn std_msgs__msg__Int32__Sequence__init(msg: *mut Int32Sequence, size: usize) -> bool;
    fn std_msgs__msg__Int32__Sequence__fini(msg: *mut Int32Sequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Int32 {
    pub data: i32,
}

impl Int32 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int32__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int32 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int32__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int32Sequence {
    data: *mut Int32,
    size: usize,
    capacity: usize,
}

impl Int32Sequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int32__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int32]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int32]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int32Sequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int32__Sequence__fini(self) };
    }
}

impl TopicMsg for Int32 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32()
        }
    }
}
