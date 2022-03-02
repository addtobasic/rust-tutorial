fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");

  &s
}

// 上記プログラムはエラー
// Rustはコンパイラが参照が段グリングにならないように保証してくれる
