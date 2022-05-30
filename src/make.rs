use crate::data::Datx;
use crate::data::From;
use crate::data::Load;
use crate::data::Save;

struct Parser {
  args: Vec<String>,
  ignore_first: bool,
  from: Vec<From>,
  load: Vec<Load>,
  save: Vec<Save>,
}

impl Parser {
  pub fn new(args: Vec<String>, ignore_first: bool) -> Self {
    Parser {
      args,
      ignore_first,
      from: Vec::new(),
      load: Vec::new(),
      save: Vec::new(),
    }
  }

  fn parse(&mut self) {}

  fn get(self) -> Datx {
    Datx {
      from: self.from,
      load: self.load,
      save: self.save,
    }
  }
}

pub fn parse(args: Vec<String>, ignore_first: bool) -> Datx {
  let mut parser = Parser::new(args, ignore_first);
  parser.parse();
  parser.get()
}
