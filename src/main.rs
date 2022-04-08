use std::io;
use rand::Rng;

fn main() {
  
  println!("Sorag sany: ");
  let questions = read_line_i32()[0];
  let mut correct: i32 = 0;
  let mut error: i32 = 0;

  let mut rng = rand::thread_rng();
  for _ in 0..questions {
    let a: i32 = rng.gen_range(1..20);
    let b: i32 = rng.gen_range(1..20);

    println!("{} + {} = ", a, b);
    let ans: i32 = read_line_i32()[0];
    println!();

    if (a + b) == ans {
      println!("Dogry");
      correct += 1;
    } else {
      println!("Nädogry");
      error += 1;
    }
  }

  println!("Dogry jogaplaryň sany = {}\nNädogry jogaplaryň sany = {}", correct, error);
}

fn read_line_i32() -> Vec<i32> {
  let handle = io::stdin();
  let mut input = String::new();
  handle.read_line(&mut input).unwrap();
  let res: Vec<i32> = input.split_whitespace()
          .map(|s| s.parse::<i32>().unwrap())
          .collect();

  res
}
