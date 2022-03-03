let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
};

// 構造体更新
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  active: user1.active,
  sign_in_count: user1.sign_in_count,
}

// 構造体更新記法(上のプログラムと同じ結果)
// emailとusernameは別の値を使い残りはuser1の値を使っている
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  ..user1
}

// タプル構造体
// フィールドに紐付けられた名前がなく型だけを定義する
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
