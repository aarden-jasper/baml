class Address {
  street string
  city string
  country string
}

class Person {
  name string
  addresses Address[]
  primary_address Address
}

class Company {
  name string
  employees Person[]
  headquarters Address
}