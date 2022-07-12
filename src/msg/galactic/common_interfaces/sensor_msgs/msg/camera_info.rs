// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__CameraInfo__init(msg: *mut CameraInfo) -> bool;
    fn sensor_msgs__msg__CameraInfo__fini(msg: *mut CameraInfo);
    fn sensor_msgs__msg__CameraInfo__Sequence__init(msg: *mut CameraInfoSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__CameraInfo__Sequence__fini(msg: *mut CameraInfoSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CameraInfo() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct CameraInfo {
    pub header: std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub distortion_model: crate::msg::RosString<0>,
    pub d: crate::msg::F64Seq<0>,
    pub k: [f64; 9],
    pub r: [f64; 9],
    pub p: [f64; 12],
    pub binning_x: u32,
    pub binning_y: u32,
    pub roi: RegionOfInterest,
}

impl CameraInfo {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__CameraInfo__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for CameraInfo {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__CameraInfo__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct CameraInfoSeqRaw {
    data: *mut CameraInfo,
    size: usize,
    capacity: usize,
}

/// Sequence of CameraInfo.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct CameraInfoSeq<const N: usize> {
    data: *mut CameraInfo,
    size: usize,
    capacity: usize,
}

impl<const N: usize> CameraInfoSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: CameraInfoSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__CameraInfo__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[CameraInfo]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [CameraInfo]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for CameraInfoSeq<N> {
    fn drop(&mut self) {
        let mut msg = CameraInfoSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__CameraInfo__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for CameraInfoSeq<N> {}
unsafe impl<const N: usize> Sync for CameraInfoSeq<N> {}


impl TopicMsg for CameraInfo {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CameraInfo()
        }
    }
}
