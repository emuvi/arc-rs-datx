use regex::Regex;

use crate::ways::*;

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

  fn parse_load(&mut self) {
    let mut name = String::default();
    let mut from = LoadFrom::All;
    let mut hunt = Regex::new(".*").unwrap();
    let mut zone = Zone::OnBody;
    let mut delta = 0;
    let mut inc_by = 1;
    let mut get_part_of_from_or_break = || {
      delta += 1;
      inc_by += 1;
      let arg = self.get_arg_by(delta);
      if arg == "from" || arg == "load" || arg == "save" || arg == "" {
        delta -= 1;
        inc_by -= 1;
        false
      } else if arg.starts_with("r'") {
        hunt = Regex::new(&arg[2..arg.len() - 1]).unwrap();
        true
      } else if arg.starts_with("--") {
        if arg == "--all" {
          from = LoadFrom::All;
        } else if arg == "--the" {
          delta += 1;
          inc_by += 1;
          from = LoadFrom::The(self.get_arg_by(delta).into())
        } else if arg == "--on-body" {
          zone = Zone::OnBody;
        } else if arg == "--on-line" {
          zone = Zone::OnLine;
        } else if arg == "--on-load" {
          delta += 1;
          inc_by += 1;
          zone = Zone::OnLoad(self.get_arg_by(delta).into());
        } else {
          panic!("Unknown load kind: {}", arg);
        }
        true
      } else {
        name = arg.into();
        true
      }
    };
    while get_part_of_from_or_break() {}
    self.load.push(Load {
      name,
      from,
      hunt,
      zone,
    });
    self.inc_index(inc_by);
  }

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
