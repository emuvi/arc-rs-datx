use crate::data::*;

struct Parser {
  args: Vec<String>,
  ignore_first: bool,
  index: usize,
  from: Vec<From>,
  load: Vec<Load>,
  save: Vec<Save>,
}

impl Parser {
  pub fn new(args: Vec<String>, ignore_first: bool) -> Self {
    Parser {
      args,
      ignore_first,
      index: 0,
      from: Vec::new(),
      load: Vec::new(),
      save: Vec::new(),
    }
  }

  fn parse_all(&mut self) {
    if self.ignore_first {
      self.index += 1;
    }
    while self.index < self.size() {
      let arg_on = self.get_arg_on();
      match arg_on {
        "from" => self.parse_from(),
        "load" => self.parse_load(),
        "save" => self.parse_save(),
        _ => panic!("Unknown command: {}", arg_on),
      }
    }
  }

  fn parse_from(&mut self) {
    let mut delta = 1;
    let mut inc_by = 2;
    let first = self.get_arg_by(delta);
    let (name, kind) = if first.starts_with("--") {
      let name = String::default();
      let kind = first;
      (name, kind)
    } else {
      let name = String::from(first);
      delta += 1;
      inc_by += 1;
      let kind = self.get_arg_by(delta);
      (name, kind)
    };
    let kind = match kind {
      "--path" => {
        delta += 1;
        inc_by += 1;
        FromKind::Path(self.get_arg_by(delta).into())
      }
      "--stdin-path" => FromKind::StdInPath,
      "--stdin-body" => FromKind::StdInBody,
      _ => panic!("Unknown from kind: {}", first),
    };
    self.from.push(From { name, kind });
    self.inc_index(inc_by);
  }

  fn parse_load(&mut self) {}

  fn parse_save(&mut self) {}

  fn inc_index(&mut self, by: usize) {
    self.index += by;
  }

  fn get_arg_on(&self) -> &str {
    &self.args[self.index]
  }

  fn get_arg_by(&self, delta: usize) -> &str {
    let arg_by = self.index + delta;
    if arg_by >= self.size() {
      return "";
    }
    &self.args[arg_by]
  }

  fn size(&self) -> usize {
    self.args.len()
  }

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
  parser.parse_all();
  parser.get()
}
