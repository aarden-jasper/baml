type PositiveInt = int @check(positive, {{ this > 0 }})
type Email = string @assert({{ this|regex_match("^[^@]+@[^@]+$") }})
type NonEmptyString = string @check(not_empty, {{ this|length > 0 }})

class ValidatedUser {
  age PositiveInt
  email Email
  name NonEmptyString @check(max_length, {{ this|length <= 100 }})
}

type BoundedInt = int @check(range, {{ this >= 0 and this <= 100 }})
type HttpUrl = string @assert({{ this|regex_match("^https?://") }})