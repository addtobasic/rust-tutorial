// 5-1
struct User {
  username: String,
  email: String,
  sign_in_count:u64,
  active: bool,
}

// 5-2
let user1 = User {
  username: String::from("hoge"),
  email: String::from("hoge@gmail.com"),
  active: true,
  sign_in_count: 1,
}

// 5-3
// インスタンス全体が可変でなければならなく, 一部のフィールドのみ可変にすることはできない
let mut user2 = User {
  username: String::from("hoge"),
  email: String::from("hoge@gmail.com"),
  active: true,
  sign_in_count: 1,
}

user1.email = String::from("anotheremail@gmail.com")

