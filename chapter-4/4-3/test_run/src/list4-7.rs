// 4-8
fn main(){
  let mut s = String::new("hello world");

  let word = first_word(&s);

  s.clear();
}

// 4-7
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b ' ' {
      return i;
    }
  }

  s.len()
}
