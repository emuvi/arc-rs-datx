use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  datx::plan::parse(args, true).start();
}
