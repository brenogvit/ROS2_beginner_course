// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from my_robot_interfaces:msg/AliveTurtles.idl
// generated code does not contain a copyright notice

#include "my_robot_interfaces/msg/detail/alive_turtles__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_my_robot_interfaces
const rosidl_type_hash_t *
my_robot_interfaces__msg__AliveTurtles__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0xd4, 0xea, 0xf3, 0xeb, 0xa0, 0x32, 0x65, 0x27,
      0xc9, 0xeb, 0x64, 0xe7, 0x33, 0x90, 0xe8, 0x14,
      0x61, 0x0e, 0x59, 0xe5, 0x65, 0xc5, 0x6d, 0xfe,
      0xc9, 0x01, 0x5e, 0x87, 0xed, 0xee, 0xfc, 0xfd,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types

// Hashes for external referenced types
#ifndef NDEBUG
#endif

static char my_robot_interfaces__msg__AliveTurtles__TYPE_NAME[] = "my_robot_interfaces/msg/AliveTurtles";

// Define type names, field names, and default values
static char my_robot_interfaces__msg__AliveTurtles__FIELD_NAME__name[] = "name";
static char my_robot_interfaces__msg__AliveTurtles__FIELD_NAME__x[] = "x";
static char my_robot_interfaces__msg__AliveTurtles__FIELD_NAME__y[] = "y";

static rosidl_runtime_c__type_description__Field my_robot_interfaces__msg__AliveTurtles__FIELDS[] = {
  {
    {my_robot_interfaces__msg__AliveTurtles__FIELD_NAME__name, 4, 4},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_STRING,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {my_robot_interfaces__msg__AliveTurtles__FIELD_NAME__x, 1, 1},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_FLOAT,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {my_robot_interfaces__msg__AliveTurtles__FIELD_NAME__y, 1, 1},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_FLOAT,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
my_robot_interfaces__msg__AliveTurtles__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {my_robot_interfaces__msg__AliveTurtles__TYPE_NAME, 36, 36},
      {my_robot_interfaces__msg__AliveTurtles__FIELDS, 3, 3},
    },
    {NULL, 0, 0},
  };
  if (!constructed) {
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "string name\n"
  "float32 x\n"
  "float32 y";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
my_robot_interfaces__msg__AliveTurtles__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {my_robot_interfaces__msg__AliveTurtles__TYPE_NAME, 36, 36},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 31, 31},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
my_robot_interfaces__msg__AliveTurtles__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[1];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 1, 1};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *my_robot_interfaces__msg__AliveTurtles__get_individual_type_description_source(NULL),
    constructed = true;
  }
  return &source_sequence;
}
