// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn std_msgs__msg__Int8__init(msg: *mut Int8) -> bool;
    fn std_msgs__msg__Int8__fini(msg: *mut Int8);
    fn std_msgs__msg__Int8__Sequence__init(msg: *mut Int8Sequence, size: usize) -> bool;
    fn std_msgs__msg__Int8__Sequence__fini(msg: *mut Int8Sequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct Int8 {
    pub data: i8,
}

impl Int8 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int8__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int8 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int8__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int8Sequence {
    data: *mut Int8,
    size: usize,
    capacity: usize,
}

impl Int8Sequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int8__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int8]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int8]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int8Sequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int8__Sequence__fini(self) };
    }
}
