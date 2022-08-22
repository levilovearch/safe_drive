// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Point32__init(msg: *mut Point32) -> bool;
    fn geometry_msgs__msg__Point32__fini(msg: *mut Point32);
    fn geometry_msgs__msg__Point32__are_equal(lhs: *const Point32, rhs: *const Point32) -> bool;
    fn geometry_msgs__msg__Point32__Sequence__init(msg: *mut Point32SeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__Point32__Sequence__fini(msg: *mut Point32SeqRaw);
    fn geometry_msgs__msg__Point32__Sequence__are_equal(
        lhs: *const Point32SeqRaw,
        rhs: *const Point32SeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point32(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point32 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Point32__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Point32 {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Point32__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct Point32SeqRaw {
    data: *mut Point32,
    size: usize,
    capacity: usize,
}

/// Sequence of Point32.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct Point32Seq<const N: usize> {
    data: *mut Point32,
    size: usize,
    capacity: usize,
}

impl<const N: usize> Point32Seq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: Point32SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Point32__Sequence__init(&mut msg, size) } {
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
        let msg: Point32SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[Point32] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Point32] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Point32> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Point32> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for Point32Seq<N> {
    fn drop(&mut self) {
        let mut msg = Point32SeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { geometry_msgs__msg__Point32__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for Point32Seq<N> {}
unsafe impl<const N: usize> Sync for Point32Seq<N> {}

impl TopicMsg for Point32 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point32()
        }
    }
}

impl PartialEq for Point32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { geometry_msgs__msg__Point32__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for Point32Seq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = Point32SeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = Point32SeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            geometry_msgs__msg__Point32__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
