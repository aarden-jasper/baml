enum Color {
  RED
  GREEN
  BLUE
}

enum Status {
  ACTIVE @alias("active_status")
  INACTIVE @description("User is not active")
  PENDING @skip
}

enum LargeEnum {
  VALUE_001
  VALUE_002
  VALUE_003
  VALUE_004
  VALUE_005
  VALUE_006
  VALUE_007
  VALUE_008
  VALUE_009
  VALUE_010
  VALUE_011
  VALUE_012
  VALUE_013
  VALUE_014
  VALUE_015
  VALUE_016
  VALUE_017
  VALUE_018
  VALUE_019
  VALUE_020
}