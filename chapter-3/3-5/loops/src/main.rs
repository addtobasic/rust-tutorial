fn main() {
  loop {
    println!("again!");
    break;
  }

  let mut counter = 3;
  while counter != 0 {
    println!("{}", counter);
    counter -= 1;
  }
  println!("LIFTOFF!!!");

  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < a.len() {
    println!("the value is: {}", a[index]);
    index += 1;
  }

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for element in (1..4).rev() {
    println!("the value is: {}", element);
  }
}
