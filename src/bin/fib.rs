use std::io;

fn fib(n: u32) -> u32 {
  if n <= 1 {
    return n;
  } else {
    return fib(n-1) + fib(n-2);
  }
}


fn main() {
  println!("Enter the Fibonacci position");
  let mut input = String::new();

  io::stdin().read_line(&mut input).expect("Enter fibonacci position!");

  let position: u32 = input.trim().parse().expect("Enter number");

  let res = fib(position);

  println!("Fibonacci: {}", res);
}