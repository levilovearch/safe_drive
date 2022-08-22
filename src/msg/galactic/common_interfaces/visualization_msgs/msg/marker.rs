// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const ARROW: i32 = 0;
pub const CUBE: i32 = 1;
pub const SPHERE: i32 = 2;
pub const CYLINDER: i32 = 3;
pub const LINE_STRIP: i32 = 4;
pub const LINE_LIST: i32 = 5;
pub const CUBE_LIST: i32 = 6;
pub const SPHERE_LIST: i32 = 7;
pub const POINTS: i32 = 8;
pub const TEXT_VIEW_FACING: i32 = 9;
pub const MESH_RESOURCE: i32 = 10;
pub const TRIANGLE_LIST: i32 = 11;
pub const ADD: i32 = 0;
pub const MODIFY: i32 = 0;
pub const DELETE: i32 = 2;
pub const DELETEALL: i32 = 3;

extern "C" {
    fn visualization_msgs__msg__Marker__init(msg: *mut Marker) -> bool;
    fn visualization_msgs__msg__Marker__fini(msg: *mut Marker);
    fn visualization_msgs__msg__Marker__are_equal(lhs: *const Marker, rhs: *const Marker) -> bool;
    fn visualization_msgs__msg__Marker__Sequence__init(msg: *mut MarkerSeqRaw, size: usize)
        -> bool;
    fn visualization_msgs__msg__Marker__Sequence__fini(msg: *mut MarkerSeqRaw);
    fn visualization_msgs__msg__Marker__Sequence__are_equal(
        lhs: *const MarkerSeqRaw,
        rhs: *const MarkerSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__Marker(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Marker {
    pub header: std_msgs::msg::Header,
    pub ns: crate::msg::RosString<0>,
    pub id: i32,
    pub type_: i32,
    pub action: i32,
    pub pose: geometry_msgs::msg::Pose,
    pub scale: geometry_msgs::msg::Vector3,
    pub color: std_msgs::msg::ColorRGBA,
    pub lifetime: builtin_interfaces::UnsafeDuration,
    pub frame_locked: bool,
    pub points: geometry_msgs::msg::PointSeq<0>,
    pub colors: std_msgs::msg::ColorRGBASeq<0>,
    pub text: crate::msg::RosString<0>,
    pub mesh_resource: crate::msg::RosString<0>,
    pub mesh_use_embedded_materials: bool,
}

impl Marker {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__Marker__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__Marker__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MarkerSeqRaw {
    data: *mut Marker,
    size: usize,
    capacity: usize,
}

/// Sequence of Marker.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MarkerSeq<const N: usize> {
    data: *mut Marker,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MarkerSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MarkerSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__Marker__Sequence__init(&mut msg, size) } {
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
        let msg: MarkerSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[Marker] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Marker] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Marker> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Marker> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for MarkerSeq<N> {
    fn drop(&mut self) {
        let mut msg = MarkerSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { visualization_msgs__msg__Marker__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MarkerSeq<N> {}
unsafe impl<const N: usize> Sync for MarkerSeq<N> {}

impl TopicMsg for Marker {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__Marker()
        }
    }
}

impl PartialEq for Marker {
    fn eq(&self, other: &Self) -> bool {
        unsafe { visualization_msgs__msg__Marker__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MarkerSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MarkerSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MarkerSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            visualization_msgs__msg__Marker__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
