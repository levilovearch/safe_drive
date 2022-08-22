// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn composition_interfaces__srv__UnloadNode_Request__init(msg: *mut UnloadNodeRequest) -> bool;
    fn composition_interfaces__srv__UnloadNode_Request__fini(msg: *mut UnloadNodeRequest);
    fn composition_interfaces__srv__UnloadNode_Request__Sequence__init(
        msg: *mut UnloadNodeRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn composition_interfaces__srv__UnloadNode_Request__Sequence__fini(
        msg: *mut UnloadNodeRequestSeqRaw,
    );
    fn composition_interfaces__srv__UnloadNode_Response__init(msg: *mut UnloadNodeResponse)
        -> bool;
    fn composition_interfaces__srv__UnloadNode_Response__fini(msg: *mut UnloadNodeResponse);
    fn composition_interfaces__srv__UnloadNode_Response__Sequence__init(
        msg: *mut UnloadNodeResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn composition_interfaces__srv__UnloadNode_Response__Sequence__fini(
        msg: *mut UnloadNodeResponseSeqRaw,
    );
    fn rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__UnloadNode(
    ) -> *const rcl::rosidl_service_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct UnloadNodeRequest {
    pub unique_id: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct UnloadNodeResponse {
    pub success: bool,
    pub error_message: crate::msg::RosString<0>,
}

impl UnloadNodeRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__UnloadNode_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for UnloadNodeRequest {
    fn drop(&mut self) {
        unsafe { composition_interfaces__srv__UnloadNode_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct UnloadNodeRequestSeqRaw {
    data: *mut UnloadNodeRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of UnloadNodeRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct UnloadNodeRequestSeq<const N: usize> {
    data: *mut UnloadNodeRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> UnloadNodeRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: UnloadNodeRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            composition_interfaces__srv__UnloadNode_Request__Sequence__init(&mut msg, size)
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
        let msg: UnloadNodeRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[UnloadNodeRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [UnloadNodeRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, UnloadNodeRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, UnloadNodeRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for UnloadNodeRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = UnloadNodeRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { composition_interfaces__srv__UnloadNode_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for UnloadNodeRequestSeq<N> {}
unsafe impl<const N: usize> Sync for UnloadNodeRequestSeq<N> {}

impl UnloadNodeResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__UnloadNode_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for UnloadNodeResponse {
    fn drop(&mut self) {
        unsafe { composition_interfaces__srv__UnloadNode_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct UnloadNodeResponseSeqRaw {
    data: *mut UnloadNodeResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of UnloadNodeResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct UnloadNodeResponseSeq<const N: usize> {
    data: *mut UnloadNodeResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> UnloadNodeResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: UnloadNodeResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            composition_interfaces__srv__UnloadNode_Response__Sequence__init(&mut msg, size)
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
        let msg: UnloadNodeResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[UnloadNodeResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [UnloadNodeResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, UnloadNodeResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, UnloadNodeResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for UnloadNodeResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = UnloadNodeResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { composition_interfaces__srv__UnloadNode_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for UnloadNodeResponseSeq<N> {}
unsafe impl<const N: usize> Sync for UnloadNodeResponseSeq<N> {}

pub struct UnloadNode;

impl ServiceMsg for UnloadNode {
    type Request = UnloadNodeRequest;
    type Response = UnloadNodeResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__UnloadNode()
        }
    }
}
