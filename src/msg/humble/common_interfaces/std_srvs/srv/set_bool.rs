// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn std_srvs__srv__SetBool_Request__init(msg: *mut SetBoolRequest) -> bool;
    fn std_srvs__srv__SetBool_Request__fini(msg: *mut SetBoolRequest);
    fn std_srvs__srv__SetBool_Request__Sequence__init(msg: *mut SetBoolRequestSeqRaw, size: usize) -> bool;
    fn std_srvs__srv__SetBool_Request__Sequence__fini(msg: *mut SetBoolRequestSeqRaw);
    fn std_srvs__srv__SetBool_Response__init(msg: *mut SetBoolResponse) -> bool;
    fn std_srvs__srv__SetBool_Response__fini(msg: *mut SetBoolResponse);
    fn std_srvs__srv__SetBool_Response__Sequence__init(msg: *mut SetBoolResponseSeqRaw, size: usize) -> bool;
    fn std_srvs__srv__SetBool_Response__Sequence__fini(msg: *mut SetBoolResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__SetBool() -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__SetBool_Request() -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__SetBool_Response() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct SetBoolRequest {
    pub data: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct SetBoolResponse {
    pub success: bool,
    pub message: crate::msg::RosString<0>,
}

impl SetBoolRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__SetBool_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetBoolRequest {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__SetBool_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetBoolRequestSeqRaw {
    data: *mut SetBoolRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of SetBoolRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetBoolRequestSeq<const N: usize> {
    data: *mut SetBoolRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetBoolRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetBoolRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__SetBool_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: SetBoolRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[SetBoolRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetBoolRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetBoolRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetBoolRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetBoolRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetBoolRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_srvs__srv__SetBool_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetBoolRequestSeq<N> {}
unsafe impl<const N: usize> Sync for SetBoolRequestSeq<N> {}


impl SetBoolResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__SetBool_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetBoolResponse {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__SetBool_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetBoolResponseSeqRaw {
    data: *mut SetBoolResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of SetBoolResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetBoolResponseSeq<const N: usize> {
    data: *mut SetBoolResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetBoolResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetBoolResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__SetBool_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: SetBoolResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[SetBoolResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetBoolResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetBoolResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetBoolResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetBoolResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetBoolResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_srvs__srv__SetBool_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetBoolResponseSeq<N> {}
unsafe impl<const N: usize> Sync for SetBoolResponseSeq<N> {}


pub struct SetBool;

impl ServiceMsg for SetBool {
    type Request = SetBoolRequest;
    type Response = SetBoolResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__SetBool()
        }
    }
}

impl TypeSupport for SetBoolRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__SetBool_Request()
        }
    }
}

impl TypeSupport for SetBoolResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__SetBool_Response()
        }
    }
}

