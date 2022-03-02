// 文字列スライスができる
// 文字列リテラルはスライスである

fn main() {
  let s = String::from("hello world");
  let hello = &s[0..5]; // [..5]
  let world = &s[6..11]; // [6..]
}
