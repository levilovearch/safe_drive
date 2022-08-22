// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn lifecycle_msgs__srv__ChangeState_Request__init(msg: *mut ChangeStateRequest) -> bool;
    fn lifecycle_msgs__srv__ChangeState_Request__fini(msg: *mut ChangeStateRequest);
    fn lifecycle_msgs__srv__ChangeState_Request__Sequence__init(
        msg: *mut ChangeStateRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn lifecycle_msgs__srv__ChangeState_Request__Sequence__fini(msg: *mut ChangeStateRequestSeqRaw);
    fn lifecycle_msgs__srv__ChangeState_Response__init(msg: *mut ChangeStateResponse) -> bool;
    fn lifecycle_msgs__srv__ChangeState_Response__fini(msg: *mut ChangeStateResponse);
    fn lifecycle_msgs__srv__ChangeState_Response__Sequence__init(
        msg: *mut ChangeStateResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn lifecycle_msgs__srv__ChangeState_Response__Sequence__fini(
        msg: *mut ChangeStateResponseSeqRaw,
    );
    fn rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__ChangeState(
    ) -> *const rcl::rosidl_service_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct ChangeStateRequest {
    pub transition: Transition,
}

#[repr(C)]
#[derive(Debug)]
pub struct ChangeStateResponse {
    pub success: bool,
}

impl ChangeStateRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__ChangeState_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ChangeStateRequest {
    fn drop(&mut self) {
        unsafe { lifecycle_msgs__srv__ChangeState_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ChangeStateRequestSeqRaw {
    data: *mut ChangeStateRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of ChangeStateRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ChangeStateRequestSeq<const N: usize> {
    data: *mut ChangeStateRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ChangeStateRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ChangeStateRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__ChangeState_Request__Sequence__init(&mut msg, size) } {
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
        let msg: ChangeStateRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[ChangeStateRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ChangeStateRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ChangeStateRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ChangeStateRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ChangeStateRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = ChangeStateRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { lifecycle_msgs__srv__ChangeState_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ChangeStateRequestSeq<N> {}
unsafe impl<const N: usize> Sync for ChangeStateRequestSeq<N> {}

impl ChangeStateResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__ChangeState_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ChangeStateResponse {
    fn drop(&mut self) {
        unsafe { lifecycle_msgs__srv__ChangeState_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ChangeStateResponseSeqRaw {
    data: *mut ChangeStateResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of ChangeStateResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ChangeStateResponseSeq<const N: usize> {
    data: *mut ChangeStateResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ChangeStateResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ChangeStateResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__ChangeState_Response__Sequence__init(&mut msg, size) } {
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
        let msg: ChangeStateResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[ChangeStateResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ChangeStateResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ChangeStateResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ChangeStateResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ChangeStateResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = ChangeStateResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { lifecycle_msgs__srv__ChangeState_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ChangeStateResponseSeq<N> {}
unsafe impl<const N: usize> Sync for ChangeStateResponseSeq<N> {}

pub struct ChangeState;

impl ServiceMsg for ChangeState {
    type Request = ChangeStateRequest;
    type Response = ChangeStateResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__ChangeState(
            )
        }
    }
}
