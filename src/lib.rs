pub mod data;

use data::Load;
use data::Save;
pub struct Datx {
  pub load: Vec<Load>,
  pub save: Vec<Save>,
}

pub fn parse(args: Vec<String>) -> Datx {
  let mut load: Vec<Load> = Vec::new();
  let mut save: Vec<Save> = Vec::new();
  let mut index = 1;
  while index < args.len() {
    let arg = &args[index];

    index += 1;
  }
  Datx {
    load: load,
    save: save,
  }
}

pub fn start(datx: Datx) {}
