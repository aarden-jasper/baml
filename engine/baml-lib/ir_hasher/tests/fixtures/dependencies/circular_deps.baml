class User {
  name string
  posts Post[]
}

class Post {
  title string
  author User
  comments Comment[]
}

class Comment {
  text string
  author User
  post Post
}