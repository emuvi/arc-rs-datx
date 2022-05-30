use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let datx = datx::parse(args);
  println!("{:?}", datx);
}
