#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  // 出力方法が複数あるためエラーになる
  println!("rect1 is {}", rect1);

  // インスタンスの全フィールドを出力する
  println!("rect1 is {:?}", rect1);
}
