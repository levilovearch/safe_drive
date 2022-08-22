// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn actionlib_msgs__msg__GoalStatusArray__init(msg: *mut GoalStatusArray) -> bool;
    fn actionlib_msgs__msg__GoalStatusArray__fini(msg: *mut GoalStatusArray);
    fn actionlib_msgs__msg__GoalStatusArray__are_equal(
        lhs: *const GoalStatusArray,
        rhs: *const GoalStatusArray,
    ) -> bool;
    fn actionlib_msgs__msg__GoalStatusArray__Sequence__init(
        msg: *mut GoalStatusArraySeqRaw,
        size: usize,
    ) -> bool;
    fn actionlib_msgs__msg__GoalStatusArray__Sequence__fini(msg: *mut GoalStatusArraySeqRaw);
    fn actionlib_msgs__msg__GoalStatusArray__Sequence__are_equal(
        lhs: *const GoalStatusArraySeqRaw,
        rhs: *const GoalStatusArraySeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalStatusArray(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct GoalStatusArray {
    pub header: std_msgs::msg::Header,
    pub status_list: GoalStatusSeq<0>,
}

impl GoalStatusArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalStatusArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GoalStatusArray {
    fn drop(&mut self) {
        unsafe { actionlib_msgs__msg__GoalStatusArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GoalStatusArraySeqRaw {
    data: *mut GoalStatusArray,
    size: usize,
    capacity: usize,
}

/// Sequence of GoalStatusArray.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GoalStatusArraySeq<const N: usize> {
    data: *mut GoalStatusArray,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GoalStatusArraySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GoalStatusArraySeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalStatusArray__Sequence__init(&mut msg, size) } {
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
        let msg: GoalStatusArraySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GoalStatusArray] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GoalStatusArray] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, GoalStatusArray> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, GoalStatusArray> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for GoalStatusArraySeq<N> {
    fn drop(&mut self) {
        let mut msg = GoalStatusArraySeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { actionlib_msgs__msg__GoalStatusArray__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GoalStatusArraySeq<N> {}
unsafe impl<const N: usize> Sync for GoalStatusArraySeq<N> {}

impl TopicMsg for GoalStatusArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalStatusArray()
        }
    }
}

impl PartialEq for GoalStatusArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { actionlib_msgs__msg__GoalStatusArray__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for GoalStatusArraySeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = GoalStatusArraySeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = GoalStatusArraySeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            actionlib_msgs__msg__GoalStatusArray__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
