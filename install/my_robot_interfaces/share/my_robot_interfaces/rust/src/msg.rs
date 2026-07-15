#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to my_robot_interfaces__msg__HardwareStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub debug_message: std::string::String,

}



impl Default for HardwareStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HardwareStatus::default())
  }
}

impl rosidl_runtime_rs::Message for HardwareStatus {
  type RmwMsg = super::msg::rmw::HardwareStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        temperature: msg.temperature,
        are_motors_ready: msg.are_motors_ready,
        debug_message: msg.debug_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      temperature: msg.temperature,
      are_motors_ready: msg.are_motors_ready,
        debug_message: msg.debug_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      temperature: msg.temperature,
      are_motors_ready: msg.are_motors_ready,
      debug_message: msg.debug_message.to_string(),
    }
  }
}


// Corresponds to my_robot_interfaces__msg__LedPanelState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LedPanelState::default())
  }
}

impl rosidl_runtime_rs::Message for LedPanelState {
  type RmwMsg = super::msg::rmw::LedPanelState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        is_led1_on: msg.is_led1_on,
        is_led2_on: msg.is_led2_on,
        is_led3_on: msg.is_led3_on,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      is_led1_on: msg.is_led1_on,
      is_led2_on: msg.is_led2_on,
      is_led3_on: msg.is_led3_on,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      is_led1_on: msg.is_led1_on,
      is_led2_on: msg.is_led2_on,
      is_led3_on: msg.is_led3_on,
    }
  }
}


// Corresponds to my_robot_interfaces__msg__AliveTurtles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AliveTurtles {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,

}



impl Default for AliveTurtles {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AliveTurtles::default())
  }
}

impl rosidl_runtime_rs::Message for AliveTurtles {
  type RmwMsg = super::msg::rmw::AliveTurtles;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        x: msg.x,
        y: msg.y,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      x: msg.x,
      y: msg.y,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      x: msg.x,
      y: msg.y,
    }
  }
}


