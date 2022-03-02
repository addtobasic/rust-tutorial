fn main() {
  let s = String::from("hello");

  change(&s);
}

fn change(some_string: &String){
  some_string.push_str(", world");
}

// 借用した値は変更できない
