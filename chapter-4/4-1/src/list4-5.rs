fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}

// 別の変数に値を代入すると、ムーブされます。 ヒープにデータを含む変数がスコープを抜けると、データが別の変数に所有されるようムーブされていない限り、 dropにより片付けられる.
