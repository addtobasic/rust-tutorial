fn main() {
  another_function(5);

  let x = 5;

  let y = {
    let x = 3;
    x + 1
  };
  println!("x: {}, y: {}", x, y);

  let five = five();
  println!("five: {}", five);

  let plus_one = plus_one(5);
  println!("plus_one: {}", plus_one);
}

fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}

fn five() -> i32 {
  5
}

// 引数に1を足した値を返す関数
fn plus_one(x: i32) -> i32 {
  x + 1
}
