class Person {
  name string
  age int
  email string?
}

class Empty {
}

class WithDefaults {
  name string @description("The person's name")
  age int @alias("user_age")
  active bool @skip
}