use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  datx::make::parse(args, true).start();
}
