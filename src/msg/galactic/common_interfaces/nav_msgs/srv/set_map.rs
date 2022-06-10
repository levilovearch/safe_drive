// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn nav_msgs__srv__SetMap_Request__init(msg: *mut SetMapRequest) -> bool;
    fn nav_msgs__srv__SetMap_Request__fini(msg: *mut SetMapRequest);
    fn nav_msgs__srv__SetMap_Request__Sequence__init(msg: *mut SetMapRequestSequence, size: usize) -> bool;
    fn nav_msgs__srv__SetMap_Request__Sequence__fini(msg: *mut SetMapRequestSequence);
    fn nav_msgs__srv__SetMap_Response__init(msg: *mut SetMapResponse) -> bool;
    fn nav_msgs__srv__SetMap_Response__fini(msg: *mut SetMapResponse);
    fn nav_msgs__srv__SetMap_Response__Sequence__init(msg: *mut SetMapResponseSequence, size: usize) -> bool;
    fn nav_msgs__srv__SetMap_Response__Sequence__fini(msg: *mut SetMapResponseSequence);
    fn rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__SetMap() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct SetMapRequest {
    pub map: OccupancyGrid,
    pub initial_pose: geometry_msgs::msg::PoseWithCovarianceStamped,
}

#[repr(C)]
#[derive(Debug)]
pub struct SetMapResponse {
    pub success: bool,
}

impl SetMapRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__SetMap_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetMapRequest {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__SetMap_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SetMapRequestSequence {
    data: *mut SetMapRequest,
    size: usize,
    capacity: usize,
}

impl SetMapRequestSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__SetMap_Request__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SetMapRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SetMapRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for SetMapRequestSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__SetMap_Request__Sequence__fini(self) };
    }
}

impl SetMapResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__SetMap_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetMapResponse {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__SetMap_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SetMapResponseSequence {
    data: *mut SetMapResponse,
    size: usize,
    capacity: usize,
}

impl SetMapResponseSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__SetMap_Response__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SetMapResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SetMapResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for SetMapResponseSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__SetMap_Response__Sequence__fini(self) };
    }
}

pub struct SetMap;

impl ServiceMsg for SetMap {
    type Request = SetMapRequest;
    type Response = SetMapResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__SetMap()
        }
    }
}

