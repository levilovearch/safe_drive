// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn rcl_interfaces__msg__ParameterEventDescriptors__init(
        msg: *mut ParameterEventDescriptors,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEventDescriptors__fini(msg: *mut ParameterEventDescriptors);
    fn rcl_interfaces__msg__ParameterEventDescriptors__are_equal(
        lhs: *const ParameterEventDescriptors,
        rhs: *const ParameterEventDescriptors,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEventDescriptors__Sequence__init(
        msg: *mut ParameterEventDescriptorsSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEventDescriptors__Sequence__fini(
        msg: *mut ParameterEventDescriptorsSeqRaw,
    );
    fn rcl_interfaces__msg__ParameterEventDescriptors__Sequence__are_equal(
        lhs: *const ParameterEventDescriptorsSeqRaw,
        rhs: *const ParameterEventDescriptorsSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEventDescriptors(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct ParameterEventDescriptors {
    pub new_parameters: ParameterDescriptorSeq<0>,
    pub changed_parameters: ParameterDescriptorSeq<0>,
    pub deleted_parameters: ParameterDescriptorSeq<0>,
}

impl ParameterEventDescriptors {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterEventDescriptors__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ParameterEventDescriptors {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__ParameterEventDescriptors__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ParameterEventDescriptorsSeqRaw {
    data: *mut ParameterEventDescriptors,
    size: usize,
    capacity: usize,
}

/// Sequence of ParameterEventDescriptors.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ParameterEventDescriptorsSeq<const N: usize> {
    data: *mut ParameterEventDescriptors,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ParameterEventDescriptorsSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ParameterEventDescriptorsSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterEventDescriptors__Sequence__init(&mut msg, size) }
        {
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
        let msg: ParameterEventDescriptorsSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[ParameterEventDescriptors] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ParameterEventDescriptors] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ParameterEventDescriptors> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ParameterEventDescriptors> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ParameterEventDescriptorsSeq<N> {
    fn drop(&mut self) {
        let mut msg = ParameterEventDescriptorsSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__msg__ParameterEventDescriptors__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ParameterEventDescriptorsSeq<N> {}
unsafe impl<const N: usize> Sync for ParameterEventDescriptorsSeq<N> {}

impl TopicMsg for ParameterEventDescriptors {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEventDescriptors()
        }
    }
}

impl PartialEq for ParameterEventDescriptors {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rcl_interfaces__msg__ParameterEventDescriptors__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for ParameterEventDescriptorsSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = ParameterEventDescriptorsSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = ParameterEventDescriptorsSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            rcl_interfaces__msg__ParameterEventDescriptors__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
