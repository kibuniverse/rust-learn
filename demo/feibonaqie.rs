
fn main() {
  let x: u32 = 6;
  println!("{}' feibonaqie =  {}", x, feibonaqie(x));
}

fn feibonaqie(n: u32) -> u32 {
  if n <= 2 {
    return 1;
  }
  feibonaqie(n - 1) + feibonaqie(n - 2)
}