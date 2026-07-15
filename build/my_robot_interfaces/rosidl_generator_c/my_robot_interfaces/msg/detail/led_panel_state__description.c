// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from my_robot_interfaces:msg/LedPanelState.idl
// generated code does not contain a copyright notice

#include "my_robot_interfaces/msg/detail/led_panel_state__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_my_robot_interfaces
const rosidl_type_hash_t *
my_robot_interfaces__msg__LedPanelState__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0x5d, 0xa8, 0xe0, 0xbd, 0xa1, 0x67, 0xa8, 0x23,
      0xa7, 0xbf, 0x94, 0x6b, 0xf2, 0xdf, 0x3c, 0x07,
      0x14, 0xdd, 0x67, 0x0f, 0xc9, 0x75, 0x0e, 0x4d,
      0xf9, 0xe9, 0x92, 0xaa, 0x89, 0x88, 0xae, 0x53,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types

// Hashes for external referenced types
#ifndef NDEBUG
#endif

static char my_robot_interfaces__msg__LedPanelState__TYPE_NAME[] = "my_robot_interfaces/msg/LedPanelState";

// Define type names, field names, and default values
static char my_robot_interfaces__msg__LedPanelState__FIELD_NAME__is_led1_on[] = "is_led1_on";
static char my_robot_interfaces__msg__LedPanelState__FIELD_NAME__is_led2_on[] = "is_led2_on";
static char my_robot_interfaces__msg__LedPanelState__FIELD_NAME__is_led3_on[] = "is_led3_on";

static rosidl_runtime_c__type_description__Field my_robot_interfaces__msg__LedPanelState__FIELDS[] = {
  {
    {my_robot_interfaces__msg__LedPanelState__FIELD_NAME__is_led1_on, 10, 10},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {my_robot_interfaces__msg__LedPanelState__FIELD_NAME__is_led2_on, 10, 10},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {my_robot_interfaces__msg__LedPanelState__FIELD_NAME__is_led3_on, 10, 10},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
my_robot_interfaces__msg__LedPanelState__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {my_robot_interfaces__msg__LedPanelState__TYPE_NAME, 37, 37},
      {my_robot_interfaces__msg__LedPanelState__FIELDS, 3, 3},
    },
    {NULL, 0, 0},
  };
  if (!constructed) {
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "bool is_led1_on\n"
  "bool is_led2_on\n"
  "bool is_led3_on";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
my_robot_interfaces__msg__LedPanelState__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {my_robot_interfaces__msg__LedPanelState__TYPE_NAME, 37, 37},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 47, 47},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
my_robot_interfaces__msg__LedPanelState__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[1];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 1, 1};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *my_robot_interfaces__msg__LedPanelState__get_individual_type_description_source(NULL),
    constructed = true;
  }
  return &source_sequence;
}
