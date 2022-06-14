// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const ULTRASOUND: u8 = 0;
pub const INFRARED: u8 = 1;

extern "C" {
    fn sensor_msgs__msg__Range__init(msg: *mut Range) -> bool;
    fn sensor_msgs__msg__Range__fini(msg: *mut Range);
    fn sensor_msgs__msg__Range__Sequence__init(msg: *mut RangeSequence, size: usize) -> bool;
    fn sensor_msgs__msg__Range__Sequence__fini(msg: *mut RangeSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Range() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Range {
    pub header: std_msgs::msg::Header,
    pub radiation_type: u8,
    pub field_of_view: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub range: f32,
}

impl Range {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Range__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Range {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Range__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RangeSequence {
    data: *mut Range,
    size: usize,
    capacity: usize,
}

impl RangeSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Range__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Range]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Range]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for RangeSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Range__Sequence__fini(self) };
    }
}

impl TopicMsg for Range {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Range()
        }
    }
}