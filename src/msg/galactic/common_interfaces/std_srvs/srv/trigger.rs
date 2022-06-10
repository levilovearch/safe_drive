// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn std_srvs__srv__Trigger_Request__init(msg: *mut TriggerRequest) -> bool;
    fn std_srvs__srv__Trigger_Request__fini(msg: *mut TriggerRequest);
    fn std_srvs__srv__Trigger_Request__Sequence__init(msg: *mut TriggerRequestSequence, size: usize) -> bool;
    fn std_srvs__srv__Trigger_Request__Sequence__fini(msg: *mut TriggerRequestSequence);
    fn std_srvs__srv__Trigger_Response__init(msg: *mut TriggerResponse) -> bool;
    fn std_srvs__srv__Trigger_Response__fini(msg: *mut TriggerResponse);
    fn std_srvs__srv__Trigger_Response__Sequence__init(msg: *mut TriggerResponseSequence, size: usize) -> bool;
    fn std_srvs__srv__Trigger_Response__Sequence__fini(msg: *mut TriggerResponseSequence);
    fn rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Trigger() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct TriggerRequest {
    _unused: u8
}

#[repr(C)]
#[derive(Debug)]
pub struct TriggerResponse {
    pub success: bool,
    pub message: crate::msg::RosString<0>,
}

impl TriggerRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TriggerRequest {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Trigger_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TriggerRequestSequence {
    data: *mut TriggerRequest,
    size: usize,
    capacity: usize,
}

impl TriggerRequestSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Request__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TriggerRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TriggerRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for TriggerRequestSequence {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Trigger_Request__Sequence__fini(self) };
    }
}

impl TriggerResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TriggerResponse {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Trigger_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TriggerResponseSequence {
    data: *mut TriggerResponse,
    size: usize,
    capacity: usize,
}

impl TriggerResponseSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Response__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TriggerResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TriggerResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for TriggerResponseSequence {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Trigger_Response__Sequence__fini(self) };
    }
}

pub struct Trigger;

impl ServiceMsg for Trigger {
    type Request = TriggerRequest;
    type Response = TriggerResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Trigger()
        }
    }
}

