#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__ComputeRectangleArea_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__ComputeRectangleArea_Request__init(msg: *mut ComputeRectangleArea_Request) -> bool;
    fn my_robot_interfaces__srv__ComputeRectangleArea_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeRectangleArea_Request>, size: usize) -> bool;
    fn my_robot_interfaces__srv__ComputeRectangleArea_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeRectangleArea_Request>);
    fn my_robot_interfaces__srv__ComputeRectangleArea_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeRectangleArea_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeRectangleArea_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__ComputeRectangleArea_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeRectangleArea_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub length: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub width: f64,

}



impl Default for ComputeRectangleArea_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__ComputeRectangleArea_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__ComputeRectangleArea_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeRectangleArea_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__ComputeRectangleArea_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__ComputeRectangleArea_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__ComputeRectangleArea_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeRectangleArea_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeRectangleArea_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/ComputeRectangleArea_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__ComputeRectangleArea_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__ComputeRectangleArea_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__ComputeRectangleArea_Response__init(msg: *mut ComputeRectangleArea_Response) -> bool;
    fn my_robot_interfaces__srv__ComputeRectangleArea_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeRectangleArea_Response>, size: usize) -> bool;
    fn my_robot_interfaces__srv__ComputeRectangleArea_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeRectangleArea_Response>);
    fn my_robot_interfaces__srv__ComputeRectangleArea_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeRectangleArea_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeRectangleArea_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__ComputeRectangleArea_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeRectangleArea_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub area: f64,

}



impl Default for ComputeRectangleArea_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__ComputeRectangleArea_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__ComputeRectangleArea_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeRectangleArea_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__ComputeRectangleArea_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__ComputeRectangleArea_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__ComputeRectangleArea_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeRectangleArea_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeRectangleArea_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/ComputeRectangleArea_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__ComputeRectangleArea_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__SetLed_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__SetLed_Request__init(msg: *mut SetLed_Request) -> bool;
    fn my_robot_interfaces__srv__SetLed_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLed_Request>, size: usize) -> bool;
    fn my_robot_interfaces__srv__SetLed_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLed_Request>);
    fn my_robot_interfaces__srv__SetLed_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLed_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLed_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__SetLed_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLed_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub led_number: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: bool,

}



impl Default for SetLed_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__SetLed_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__SetLed_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLed_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__SetLed_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__SetLed_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__SetLed_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLed_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLed_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/SetLed_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__SetLed_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__SetLed_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__SetLed_Response__init(msg: *mut SetLed_Response) -> bool;
    fn my_robot_interfaces__srv__SetLed_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLed_Response>, size: usize) -> bool;
    fn my_robot_interfaces__srv__SetLed_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLed_Response>);
    fn my_robot_interfaces__srv__SetLed_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLed_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLed_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__SetLed_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLed_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetLed_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__SetLed_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__SetLed_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLed_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__SetLed_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__SetLed_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__SetLed_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLed_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLed_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/SetLed_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__SetLed_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__CatchTurtle_Request__init(msg: *mut CatchTurtle_Request) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Request>, size: usize) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Request>);
    fn my_robot_interfaces__srv__CatchTurtle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CatchTurtle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__CatchTurtle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CatchTurtle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CatchTurtle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__CatchTurtle_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__CatchTurtle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CatchTurtle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CatchTurtle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CatchTurtle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/CatchTurtle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__CatchTurtle_Response__init(msg: *mut CatchTurtle_Response) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Response>, size: usize) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Response>);
    fn my_robot_interfaces__srv__CatchTurtle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CatchTurtle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__CatchTurtle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CatchTurtle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for CatchTurtle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__CatchTurtle_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__CatchTurtle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CatchTurtle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CatchTurtle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CatchTurtle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/CatchTurtle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Response() }
  }
}






#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__ComputeRectangleArea() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__srv__ComputeRectangleArea
#[allow(missing_docs, non_camel_case_types)]
pub struct ComputeRectangleArea;

impl rosidl_runtime_rs::Service for ComputeRectangleArea {
    type Request = ComputeRectangleArea_Request;
    type Response = ComputeRectangleArea_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__ComputeRectangleArea() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__SetLed() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__srv__SetLed
#[allow(missing_docs, non_camel_case_types)]
pub struct SetLed;

impl rosidl_runtime_rs::Service for SetLed {
    type Request = SetLed_Request;
    type Response = SetLed_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__SetLed() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__CatchTurtle() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__srv__CatchTurtle
#[allow(missing_docs, non_camel_case_types)]
pub struct CatchTurtle;

impl rosidl_runtime_rs::Service for CatchTurtle {
    type Request = CatchTurtle_Request;
    type Response = CatchTurtle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__CatchTurtle() }
    }
}


