// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn diagnostic_msgs__msg__DiagnosticArray__init(msg: *mut DiagnosticArray) -> bool;
    fn diagnostic_msgs__msg__DiagnosticArray__fini(msg: *mut DiagnosticArray);
    fn diagnostic_msgs__msg__DiagnosticArray__Sequence__init(msg: *mut DiagnosticArraySequence, size: usize) -> bool;
    fn diagnostic_msgs__msg__DiagnosticArray__Sequence__fini(msg: *mut DiagnosticArraySequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticArray {
    pub header: std_msgs::msg::Header,
    pub status: DiagnosticStatusSequence,
}

impl DiagnosticArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for DiagnosticArray {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__DiagnosticArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticArraySequence {
    data: *mut DiagnosticArray,
    size: usize,
    capacity: usize,
}

impl DiagnosticArraySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticArray__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[DiagnosticArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [DiagnosticArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for DiagnosticArraySequence {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__DiagnosticArray__Sequence__fini(self) };
    }
}

impl TopicMsg for DiagnosticArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticArray()
        }
    }
}
