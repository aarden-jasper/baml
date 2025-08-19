// Test @alias attribute (implementation only)
class PersonWithAlias {
  first_name string @alias("firstName")
  last_name string @alias("lastName")
}

// Test @description attribute (implementation only)
enum StatusWithDescription {
  ACTIVE @description("User is currently active")
  INACTIVE @description("User is not active")  
  PENDING @description("User is pending activation")
}

// Test @skip attribute (implementation only)
class ConfigWithSkip {
  api_key string @skip
  public_data string
}