// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn actionlib_msgs__msg__GoalID__init(msg: *mut GoalID) -> bool;
    fn actionlib_msgs__msg__GoalID__fini(msg: *mut GoalID);
    fn actionlib_msgs__msg__GoalID__are_equal(lhs: *const GoalID, rhs: *const GoalID) -> bool;
    fn actionlib_msgs__msg__GoalID__Sequence__init(msg: *mut GoalIDSeqRaw, size: usize) -> bool;
    fn actionlib_msgs__msg__GoalID__Sequence__fini(msg: *mut GoalIDSeqRaw);
    fn actionlib_msgs__msg__GoalID__Sequence__are_equal(
        lhs: *const GoalIDSeqRaw,
        rhs: *const GoalIDSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalID(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct GoalID {
    pub stamp: builtin_interfaces::UnsafeTime,
    pub id: crate::msg::RosString<0>,
}

impl GoalID {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalID__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GoalID {
    fn drop(&mut self) {
        unsafe { actionlib_msgs__msg__GoalID__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GoalIDSeqRaw {
    data: *mut GoalID,
    size: usize,
    capacity: usize,
}

/// Sequence of GoalID.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GoalIDSeq<const N: usize> {
    data: *mut GoalID,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GoalIDSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GoalIDSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalID__Sequence__init(&mut msg, size) } {
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
        let msg: GoalIDSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GoalID] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GoalID] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, GoalID> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, GoalID> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for GoalIDSeq<N> {
    fn drop(&mut self) {
        let mut msg = GoalIDSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { actionlib_msgs__msg__GoalID__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GoalIDSeq<N> {}
unsafe impl<const N: usize> Sync for GoalIDSeq<N> {}

impl TopicMsg for GoalID {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalID()
        }
    }
}

impl PartialEq for GoalID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { actionlib_msgs__msg__GoalID__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for GoalIDSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = GoalIDSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = GoalIDSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            actionlib_msgs__msg__GoalID__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
