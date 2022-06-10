// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn nav_msgs__srv__GetMap_Request__init(msg: *mut GetMapRequest) -> bool;
    fn nav_msgs__srv__GetMap_Request__fini(msg: *mut GetMapRequest);
    fn nav_msgs__srv__GetMap_Request__Sequence__init(msg: *mut GetMapRequestSequence, size: usize) -> bool;
    fn nav_msgs__srv__GetMap_Request__Sequence__fini(msg: *mut GetMapRequestSequence);
    fn nav_msgs__srv__GetMap_Response__init(msg: *mut GetMapResponse) -> bool;
    fn nav_msgs__srv__GetMap_Response__fini(msg: *mut GetMapResponse);
    fn nav_msgs__srv__GetMap_Response__Sequence__init(msg: *mut GetMapResponseSequence, size: usize) -> bool;
    fn nav_msgs__srv__GetMap_Response__Sequence__fini(msg: *mut GetMapResponseSequence);
    fn rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__GetMap() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct GetMapRequest {
    _unused: u8
}

#[repr(C)]
#[derive(Debug)]
pub struct GetMapResponse {
    pub map: OccupancyGrid,
}

impl GetMapRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetMap_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetMapRequest {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__GetMap_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GetMapRequestSequence {
    data: *mut GetMapRequest,
    size: usize,
    capacity: usize,
}

impl GetMapRequestSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetMap_Request__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[GetMapRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [GetMapRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for GetMapRequestSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__GetMap_Request__Sequence__fini(self) };
    }
}

impl GetMapResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetMap_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetMapResponse {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__GetMap_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GetMapResponseSequence {
    data: *mut GetMapResponse,
    size: usize,
    capacity: usize,
}

impl GetMapResponseSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetMap_Response__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[GetMapResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [GetMapResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for GetMapResponseSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__GetMap_Response__Sequence__fini(self) };
    }
}

pub struct GetMap;

impl ServiceMsg for GetMap {
    type Request = GetMapRequest;
    type Response = GetMapResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__GetMap()
        }
    }
}

