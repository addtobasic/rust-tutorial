fn main() {
  let guess: u32 = "42".parse()
    .expect("Not a number");
  println!("{}", guess);

  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
  println!("x: {}, y: {},", x, y);

  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("x: {}, y: {}, z: {}", x, y, z);
  println!("x: {}, y: {}, z: {}", tup.0, tup.1, tup.2);

  let a = [1, 2, 3, 4, 5];
  println!("a[0]: {}, a[1]: {}", a[0], a[1])
}
