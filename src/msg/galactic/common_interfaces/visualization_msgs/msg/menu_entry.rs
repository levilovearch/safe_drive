// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const FEEDBACK: u8 = 0;
pub const ROSRUN: u8 = 1;
pub const ROSLAUNCH: u8 = 2;

extern "C" {
    fn visualization_msgs__msg__MenuEntry__init(msg: *mut MenuEntry) -> bool;
    fn visualization_msgs__msg__MenuEntry__fini(msg: *mut MenuEntry);
    fn visualization_msgs__msg__MenuEntry__Sequence__init(msg: *mut MenuEntrySequence, size: usize) -> bool;
    fn visualization_msgs__msg__MenuEntry__Sequence__fini(msg: *mut MenuEntrySequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__MenuEntry() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MenuEntry {
    pub id: u32,
    pub parent_id: u32,
    pub title: crate::msg::RosString<0>,
    pub command: crate::msg::RosString<0>,
    pub command_type: u8,
}

impl MenuEntry {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__MenuEntry__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MenuEntry {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__MenuEntry__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MenuEntrySequence {
    data: *mut MenuEntry,
    size: usize,
    capacity: usize,
}

impl MenuEntrySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__MenuEntry__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[MenuEntry]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [MenuEntry]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for MenuEntrySequence {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__MenuEntry__Sequence__fini(self) };
    }
}

impl TopicMsg for MenuEntry {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__MenuEntry()
        }
    }
}
