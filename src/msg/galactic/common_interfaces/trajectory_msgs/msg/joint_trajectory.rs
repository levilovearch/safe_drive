// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn trajectory_msgs__msg__JointTrajectory__init(msg: *mut JointTrajectory) -> bool;
    fn trajectory_msgs__msg__JointTrajectory__fini(msg: *mut JointTrajectory);
    fn trajectory_msgs__msg__JointTrajectory__Sequence__init(msg: *mut JointTrajectorySeqRaw, size: usize) -> bool;
    fn trajectory_msgs__msg__JointTrajectory__Sequence__fini(msg: *mut JointTrajectorySeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectory() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct JointTrajectory {
    pub header: std_msgs::msg::Header,
    pub joint_names: crate::msg::RosStringSeq<0, 0>,
    pub points: JointTrajectoryPointSeq<0>,
}

impl JointTrajectory {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { trajectory_msgs__msg__JointTrajectory__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for JointTrajectory {
    fn drop(&mut self) {
        unsafe { trajectory_msgs__msg__JointTrajectory__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct JointTrajectorySeqRaw {
    data: *mut JointTrajectory,
    size: usize,
    capacity: usize,
}

/// Sequence of JointTrajectory.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct JointTrajectorySeq<const N: usize> {
    data: *mut JointTrajectory,
    size: usize,
    capacity: usize,
}

impl<const N: usize> JointTrajectorySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: JointTrajectorySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { trajectory_msgs__msg__JointTrajectory__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[JointTrajectory]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [JointTrajectory]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for JointTrajectorySeq<N> {
    fn drop(&mut self) {
        let mut msg = JointTrajectorySeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { trajectory_msgs__msg__JointTrajectory__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for JointTrajectorySeq<N> {}
unsafe impl<const N: usize> Sync for JointTrajectorySeq<N> {}


impl TopicMsg for JointTrajectory {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectory()
        }
    }
}
