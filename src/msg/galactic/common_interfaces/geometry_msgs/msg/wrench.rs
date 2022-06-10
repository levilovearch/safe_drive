// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Wrench__init(msg: *mut Wrench) -> bool;
    fn geometry_msgs__msg__Wrench__fini(msg: *mut Wrench);
    fn geometry_msgs__msg__Wrench__Sequence__init(msg: *mut WrenchSequence, size: usize) -> bool;
    fn geometry_msgs__msg__Wrench__Sequence__fini(msg: *mut WrenchSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Wrench() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Wrench {
    pub force: Vector3,
    pub torque: Vector3,
}

impl Wrench {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Wrench__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Wrench {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Wrench__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct WrenchSequence {
    data: *mut Wrench,
    size: usize,
    capacity: usize,
}

impl WrenchSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Wrench__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Wrench]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Wrench]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for WrenchSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Wrench__Sequence__fini(self) };
    }
}

impl TopicMsg for Wrench {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Wrench()
        }
    }
}
