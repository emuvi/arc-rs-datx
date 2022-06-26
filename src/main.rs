use std::env;

#[tokio::main]
async fn main() {
  if env::args()
    .find(|arg| arg == "-h" || arg == "--help")
    .is_some()
  {
    print_usage();
  } else {
    let args: Vec<String> = env::args().collect();
    datx::plan::parse(args, true).start();
  }
}

fn print_usage() {
  println!("Datx {}", env!("CARGO_PKG_VERSION"));
  println!("Éverton M. Vieira <emuvi@outlook.com.br>");
  println!("Datx (Data Transform Toolbox) is a library and a command program that features a toolbox with a series of data extract, transformation and load functionalities.");
  println!("");
  println!("USAGE:");
  println!("    datx [MAKE...]");
  println!("");
  println!("MAKE:");
  println!("    from [name] [KIND]");
  println!("    pick [name] [HUNT] [ZONE]");
  println!("    save [ONTO]");
  println!("");
  println!("KIND:");
  println!("    --path [path]");
  println!("    --stdin-path");
  println!("    --stdin-body");
  println!("");
  println!("HUNT:");
  println!("    r'REGEX'");
  println!("");
  println!("ZONE:");
  println!("    --all-crude");
  println!("    --on-crude [name]");
  println!("    --on-cooked [name]");
  println!("");
  println!("ONTO:");
  println!("    --on-file [WORD...]");
  println!("    --on-file --body [WORD...]");
  println!("    --on-file --path [WORD...] --body [WORD...]");
  println!("    --on-file --path [WORD...]");
  println!("");
  println!("WORD:");
  println!("   [word]");
  println!("   --as [word]");
  println!("   --as-picked [name]");
  println!("   --as-all-picked");
  println!("   --done");
}
