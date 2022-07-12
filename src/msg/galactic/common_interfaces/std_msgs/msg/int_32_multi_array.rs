// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Int32MultiArray__init(msg: *mut Int32MultiArray) -> bool;
    fn std_msgs__msg__Int32MultiArray__fini(msg: *mut Int32MultiArray);
    fn std_msgs__msg__Int32MultiArray__Sequence__init(msg: *mut Int32MultiArraySeqRaw, size: usize) -> bool;
    fn std_msgs__msg__Int32MultiArray__Sequence__fini(msg: *mut Int32MultiArraySeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32MultiArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Int32MultiArray {
    pub layout: MultiArrayLayout,
    pub data: crate::msg::I32Seq<0>,
}

impl Int32MultiArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int32MultiArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int32MultiArray {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int32MultiArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct Int32MultiArraySeqRaw {
    data: *mut Int32MultiArray,
    size: usize,
    capacity: usize,
}

/// Sequence of Int32MultiArray.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct Int32MultiArraySeq<const N: usize> {
    data: *mut Int32MultiArray,
    size: usize,
    capacity: usize,
}

impl<const N: usize> Int32MultiArraySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: Int32MultiArraySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int32MultiArray__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int32MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int32MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for Int32MultiArraySeq<N> {
    fn drop(&mut self) {
        let mut msg = Int32MultiArraySeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_msgs__msg__Int32MultiArray__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for Int32MultiArraySeq<N> {}
unsafe impl<const N: usize> Sync for Int32MultiArraySeq<N> {}


impl TopicMsg for Int32MultiArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32MultiArray()
        }
    }
}
