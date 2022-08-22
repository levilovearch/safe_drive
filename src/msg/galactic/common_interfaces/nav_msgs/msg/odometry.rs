// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn nav_msgs__msg__Odometry__init(msg: *mut Odometry) -> bool;
    fn nav_msgs__msg__Odometry__fini(msg: *mut Odometry);
    fn nav_msgs__msg__Odometry__are_equal(lhs: *const Odometry, rhs: *const Odometry) -> bool;
    fn nav_msgs__msg__Odometry__Sequence__init(msg: *mut OdometrySeqRaw, size: usize) -> bool;
    fn nav_msgs__msg__Odometry__Sequence__fini(msg: *mut OdometrySeqRaw);
    fn nav_msgs__msg__Odometry__Sequence__are_equal(
        lhs: *const OdometrySeqRaw,
        rhs: *const OdometrySeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Odometry(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Odometry {
    pub header: std_msgs::msg::Header,
    pub child_frame_id: crate::msg::RosString<0>,
    pub pose: geometry_msgs::msg::PoseWithCovariance,
    pub twist: geometry_msgs::msg::TwistWithCovariance,
}

impl Odometry {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__Odometry__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Odometry {
    fn drop(&mut self) {
        unsafe { nav_msgs__msg__Odometry__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct OdometrySeqRaw {
    data: *mut Odometry,
    size: usize,
    capacity: usize,
}

/// Sequence of Odometry.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct OdometrySeq<const N: usize> {
    data: *mut Odometry,
    size: usize,
    capacity: usize,
}

impl<const N: usize> OdometrySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: OdometrySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__Odometry__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: OdometrySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[Odometry] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Odometry] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Odometry> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Odometry> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for OdometrySeq<N> {
    fn drop(&mut self) {
        let mut msg = OdometrySeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { nav_msgs__msg__Odometry__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for OdometrySeq<N> {}
unsafe impl<const N: usize> Sync for OdometrySeq<N> {}

impl TopicMsg for Odometry {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe { rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Odometry() }
    }
}

impl PartialEq for Odometry {
    fn eq(&self, other: &Self) -> bool {
        unsafe { nav_msgs__msg__Odometry__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for OdometrySeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = OdometrySeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = OdometrySeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            nav_msgs__msg__Odometry__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
