// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from my_robot_interfaces:msg/LedPanelState.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "my_robot_interfaces/msg/led_panel_state.hpp"


#ifndef MY_ROBOT_INTERFACES__MSG__DETAIL__LED_PANEL_STATE__BUILDER_HPP_
#define MY_ROBOT_INTERFACES__MSG__DETAIL__LED_PANEL_STATE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "my_robot_interfaces/msg/detail/led_panel_state__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace my_robot_interfaces
{

namespace msg
{

namespace builder
{

class Init_LedPanelState_is_led3_on
{
public:
  explicit Init_LedPanelState_is_led3_on(::my_robot_interfaces::msg::LedPanelState & msg)
  : msg_(msg)
  {}
  ::my_robot_interfaces::msg::LedPanelState is_led3_on(::my_robot_interfaces::msg::LedPanelState::_is_led3_on_type arg)
  {
    msg_.is_led3_on = std::move(arg);
    return std::move(msg_);
  }

private:
  ::my_robot_interfaces::msg::LedPanelState msg_;
};

class Init_LedPanelState_is_led2_on
{
public:
  explicit Init_LedPanelState_is_led2_on(::my_robot_interfaces::msg::LedPanelState & msg)
  : msg_(msg)
  {}
  Init_LedPanelState_is_led3_on is_led2_on(::my_robot_interfaces::msg::LedPanelState::_is_led2_on_type arg)
  {
    msg_.is_led2_on = std::move(arg);
    return Init_LedPanelState_is_led3_on(msg_);
  }

private:
  ::my_robot_interfaces::msg::LedPanelState msg_;
};

class Init_LedPanelState_is_led1_on
{
public:
  Init_LedPanelState_is_led1_on()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_LedPanelState_is_led2_on is_led1_on(::my_robot_interfaces::msg::LedPanelState::_is_led1_on_type arg)
  {
    msg_.is_led1_on = std::move(arg);
    return Init_LedPanelState_is_led2_on(msg_);
  }

private:
  ::my_robot_interfaces::msg::LedPanelState msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_robot_interfaces::msg::LedPanelState>()
{
  return my_robot_interfaces::msg::builder::Init_LedPanelState_is_led1_on();
}

}  // namespace my_robot_interfaces

#endif  // MY_ROBOT_INTERFACES__MSG__DETAIL__LED_PANEL_STATE__BUILDER_HPP_
