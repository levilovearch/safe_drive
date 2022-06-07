// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn std_msgs__msg__Float64MultiArray__init(msg: *mut Float64MultiArray) -> bool;
    fn std_msgs__msg__Float64MultiArray__fini(msg: *mut Float64MultiArray);
    fn std_msgs__msg__Float64MultiArray__Sequence__init(msg: *mut Float64MultiArraySequence, size: usize) -> bool;
    fn std_msgs__msg__Float64MultiArray__Sequence__fini(msg: *mut Float64MultiArraySequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct Float64MultiArray {
    pub layout: super::MultiArrayLayout,
    pub data: crate::msg::F64Seq<0>,
}

impl Float64MultiArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Float64MultiArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Float64MultiArray {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Float64MultiArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Float64MultiArraySequence {
    data: *mut Float64MultiArray,
    size: usize,
    capacity: usize,
}

impl Float64MultiArraySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Float64MultiArray__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Float64MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Float64MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Float64MultiArraySequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Float64MultiArray__Sequence__fini(self) };
    }
}
