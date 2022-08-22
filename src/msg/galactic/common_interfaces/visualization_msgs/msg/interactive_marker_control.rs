// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const INHERIT: u8 = 0;
pub const FIXED: u8 = 1;
pub const VIEW_FACING: u8 = 2;
pub const NONE: u8 = 0;
pub const MENU: u8 = 1;
pub const BUTTON: u8 = 2;
pub const MOVE_AXIS: u8 = 3;
pub const MOVE_PLANE: u8 = 4;
pub const ROTATE_AXIS: u8 = 5;
pub const MOVE_ROTATE: u8 = 6;

extern "C" {
    fn visualization_msgs__msg__InteractiveMarkerControl__init(
        msg: *mut InteractiveMarkerControl,
    ) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerControl__fini(msg: *mut InteractiveMarkerControl);
    fn visualization_msgs__msg__InteractiveMarkerControl__are_equal(
        lhs: *const InteractiveMarkerControl,
        rhs: *const InteractiveMarkerControl,
    ) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerControl__Sequence__init(
        msg: *mut InteractiveMarkerControlSeqRaw,
        size: usize,
    ) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerControl__Sequence__fini(
        msg: *mut InteractiveMarkerControlSeqRaw,
    );
    fn visualization_msgs__msg__InteractiveMarkerControl__Sequence__are_equal(
        lhs: *const InteractiveMarkerControlSeqRaw,
        rhs: *const InteractiveMarkerControlSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerControl(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerControl {
    pub name: crate::msg::RosString<0>,
    pub orientation: geometry_msgs::msg::Quaternion,
    pub orientation_mode: u8,
    pub MOVE_3D: u8,
    pub ROTATE_3D: u8,
    pub MOVE_ROTATE_3D: u8,
    pub interaction_mode: u8,
    pub always_visible: bool,
    pub markers: MarkerSeq<0>,
    pub independent_marker_orientation: bool,
    pub description: crate::msg::RosString<0>,
}

impl InteractiveMarkerControl {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerControl__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InteractiveMarkerControl {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarkerControl__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct InteractiveMarkerControlSeqRaw {
    data: *mut InteractiveMarkerControl,
    size: usize,
    capacity: usize,
}

/// Sequence of InteractiveMarkerControl.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerControlSeq<const N: usize> {
    data: *mut InteractiveMarkerControl,
    size: usize,
    capacity: usize,
}

impl<const N: usize> InteractiveMarkerControlSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: InteractiveMarkerControlSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            visualization_msgs__msg__InteractiveMarkerControl__Sequence__init(&mut msg, size)
        } {
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
        let msg: InteractiveMarkerControlSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[InteractiveMarkerControl] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [InteractiveMarkerControl] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, InteractiveMarkerControl> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, InteractiveMarkerControl> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for InteractiveMarkerControlSeq<N> {
    fn drop(&mut self) {
        let mut msg = InteractiveMarkerControlSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { visualization_msgs__msg__InteractiveMarkerControl__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for InteractiveMarkerControlSeq<N> {}
unsafe impl<const N: usize> Sync for InteractiveMarkerControlSeq<N> {}

impl TopicMsg for InteractiveMarkerControl {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerControl()
        }
    }
}

impl PartialEq for InteractiveMarkerControl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { visualization_msgs__msg__InteractiveMarkerControl__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for InteractiveMarkerControlSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = InteractiveMarkerControlSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = InteractiveMarkerControlSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            visualization_msgs__msg__InteractiveMarkerControl__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
