// Test @dynamic attribute (interface change)
@@dynamic
class DynamicConfig {
  setting1 string
  setting2 int
}

@@dynamic
enum DynamicStatus {
  OPTION1
  OPTION2
}