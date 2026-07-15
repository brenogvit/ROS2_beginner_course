#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__msg__HardwareStatus() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__msg__HardwareStatus__init(msg: *mut HardwareStatus) -> bool;
    fn my_robot_interfaces__msg__HardwareStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HardwareStatus>, size: usize) -> bool;
    fn my_robot_interfaces__msg__HardwareStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HardwareStatus>);
    fn my_robot_interfaces__msg__HardwareStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HardwareStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<HardwareStatus>) -> bool;
}

// Corresponds to my_robot_interfaces__msg__HardwareStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub are_motors_ready: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub debug_message: rosidl_runtime_rs::String,

}



impl Default for HardwareStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__msg__HardwareStatus__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__msg__HardwareStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HardwareStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__HardwareStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__HardwareStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__HardwareStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HardwareStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HardwareStatus where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/msg/HardwareStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__msg__HardwareStatus() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__msg__LedPanelState() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__msg__LedPanelState__init(msg: *mut LedPanelState) -> bool;
    fn my_robot_interfaces__msg__LedPanelState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedPanelState>, size: usize) -> bool;
    fn my_robot_interfaces__msg__LedPanelState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedPanelState>);
    fn my_robot_interfaces__msg__LedPanelState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedPanelState>, out_seq: *mut rosidl_runtime_rs::Sequence<LedPanelState>) -> bool;
}

// Corresponds to my_robot_interfaces__msg__LedPanelState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedPanelState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_led1_on: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_led2_on: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_led3_on: bool,

}



impl Default for LedPanelState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__msg__LedPanelState__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__msg__LedPanelState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedPanelState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__LedPanelState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__LedPanelState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__LedPanelState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedPanelState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedPanelState where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/msg/LedPanelState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__msg__LedPanelState() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__msg__AliveTurtles() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__msg__AliveTurtles__init(msg: *mut AliveTurtles) -> bool;
    fn my_robot_interfaces__msg__AliveTurtles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AliveTurtles>, size: usize) -> bool;
    fn my_robot_interfaces__msg__AliveTurtles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AliveTurtles>);
    fn my_robot_interfaces__msg__AliveTurtles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AliveTurtles>, out_seq: *mut rosidl_runtime_rs::Sequence<AliveTurtles>) -> bool;
}

// Corresponds to my_robot_interfaces__msg__AliveTurtles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AliveTurtles {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,

}



impl Default for AliveTurtles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__msg__AliveTurtles__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__msg__AliveTurtles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AliveTurtles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__AliveTurtles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__AliveTurtles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__msg__AliveTurtles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AliveTurtles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AliveTurtles where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/msg/AliveTurtles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__msg__AliveTurtles() }
  }
}


