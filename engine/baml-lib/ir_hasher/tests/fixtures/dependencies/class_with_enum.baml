enum Status {
  ACTIVE
  INACTIVE
  PENDING
}

class User {
  name string
  status Status
}

class Account {
  user User
  account_status Status
}