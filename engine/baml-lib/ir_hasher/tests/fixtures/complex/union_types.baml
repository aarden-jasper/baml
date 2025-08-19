type StringOrInt = string | int
type NullableString = string | null
type ComplexUnion = string | int[] | map<string, bool>

class Result {
  value string | int | bool
  error string?
}

class MultiUnion {
  data string | int | float | bool | null
  metadata map<string, string | int>?
}