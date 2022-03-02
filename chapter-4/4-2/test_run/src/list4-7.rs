fn main() {
  let mut s = String::from("hello");

  change(&mut s);
}

fn change(some_string: &mut String){
  some_string.push_str(", world");
}

// それぞれmutで可変な参照にすれば解決する

// 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てない
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);


// 可変と不変な参照を組み合わせることもできない
let mut s = String::from("hello");

let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！
