// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__CompressedImage__init(msg: *mut CompressedImage) -> bool;
    fn sensor_msgs__msg__CompressedImage__fini(msg: *mut CompressedImage);
    fn sensor_msgs__msg__CompressedImage__Sequence__init(msg: *mut CompressedImageSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__CompressedImage__Sequence__fini(msg: *mut CompressedImageSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CompressedImage() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct CompressedImage {
    pub header: std_msgs::msg::Header,
    pub format: crate::msg::RosString<0>,
    pub data: crate::msg::U8Seq<0>,
}

impl CompressedImage {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__CompressedImage__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for CompressedImage {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__CompressedImage__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct CompressedImageSeqRaw {
    data: *mut CompressedImage,
    size: usize,
    capacity: usize,
}

/// Sequence of CompressedImage.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct CompressedImageSeq<const N: usize> {
    data: *mut CompressedImage,
    size: usize,
    capacity: usize,
}

impl<const N: usize> CompressedImageSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: CompressedImageSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__CompressedImage__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[CompressedImage]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [CompressedImage]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for CompressedImageSeq<N> {
    fn drop(&mut self) {
        let mut msg = CompressedImageSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__CompressedImage__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for CompressedImageSeq<N> {}
unsafe impl<const N: usize> Sync for CompressedImageSeq<N> {}


impl TopicMsg for CompressedImage {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CompressedImage()
        }
    }
}
