// フィールドと変数が同名のといはフィールド初期化省略記法が使える
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
