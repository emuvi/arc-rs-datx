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
      self.go_further(1);
      match arg_on.as_ref() {
        "from" => self.parse_from(),
        "load" => self.parse_load(),
        "save" => self.parse_save(),
        _ => panic!("Unknown command: {}", arg_on),
      }
    }
  }

  fn parse_from(&mut self) {
    let first = self.get_arg_on();
    if is_closer(&first) {
      self.from.push(From {
        name: String::default(),
        kind: FromKind::StdInBody,
      });
      return;
    }
    self.go_further(1);
    let (name, kind) = if first.starts_with("--") {
      let name = String::default();
      let kind = first;
      (name, kind)
    } else {
      let name = first;
      let kind = self.get_arg_on();
      self.go_further(1);
      (name, kind)
    };
    let kind = match kind.as_ref() {
      "--path" => {
        let path = self.get_arg_on();
        self.go_further(1);
        FromKind::Path(path)
      }
      "--stdin-path" => FromKind::StdInPath,
      "--stdin-body" => FromKind::StdInBody,
      "" => FromKind::StdInBody,
      _ => panic!("Unknown from kind: {}", kind),
    };
    self.from.push(From { name, kind });
  }

  fn parse_load(&mut self) {
    let mut name = String::default();
    let mut from = LoadFrom::All;
    let mut hunt = Regex::new(".*").unwrap();
    let mut zone = Zone::OnBody;
    let mut get_part_of_from_or_break = || {
      let arg = self.get_arg_on();
      if is_closer(&arg) {
        return false;
      }
      self.go_further(1);
      if arg.starts_with("r'") {
        hunt = Regex::new(&arg[2..arg.len() - 1]).unwrap();
      } else if arg.starts_with("--") {
        if arg == "--all" {
          from = LoadFrom::All;
        } else if arg == "--the" {
          let the = self.get_arg_on();
          self.go_further(1);
          from = LoadFrom::The(the);
        } else if arg == "--on-body" {
          zone = Zone::OnBody;
        } else if arg == "--on-line" {
          zone = Zone::OnLine;
        } else if arg == "--on-load" {
          let load = self.get_arg_on();
          self.go_further(1);
          zone = Zone::OnLoad(load);
        } else {
          panic!("Unknown load kind: {}", arg);
        }
      } else {
        name = arg;
      }
      true
    };
    while get_part_of_from_or_break() {}
    self.load.push(Load {
      name,
      from,
      hunt,
      zone,
    });
  }

  fn parse_save(&mut self) {
    let place = self.get_arg_on();
    if is_closer(&place) {
      self.save.push(Save::ToFile(OnFile {
        path: vec![Word::As("output.txt".into())],
        body: vec![Word::AsAllLoad],
      }));
      return;
    }
    self.go_further(1);
    if place == "--on-file" {
      let on_file = self.get_save_on_file();
      self.save.push(Save::ToFile(on_file));
    } else {
      panic!("Unknown save place: {}", place);
    }
  }

  fn get_save_on_file(&mut self) -> OnFile {
    let mut part = self.get_arg_on();
    if !part.starts_with("--") {
      let mut body = vec![];
      let path = vec![Word::As("output.txt".into())];
      self.get_dict(&mut body);
      return OnFile { path, body };
    }
    let mut path = vec![];
    let mut body = vec![];
    loop {
      self.go_further(1);
      if part == "--path" {
        self.get_dict(&mut path);
      } else if part == "--body" {
        self.get_dict(&mut body);
      } else {
        panic!("Unknown save on file part: {}", part);
      }
      part = self.get_arg_on();
      if is_closer(&part) {
        break;
      }
    }
    return OnFile { path, body };
  }

  fn get_dict(&mut self, on: &mut Dict) {
    loop {
      let word = self.get_arg_on();
      if is_closer(&word) {
        return;
      }
      self.go_further(1);
      if word.starts_with("--") {
        if word == "--done" {
          return;
        } else if word == "--path" || word == "--body" {
          self.go_back(1);
          return;
        } else if word == "--as" {
          let like = self.get_arg_on();
          self.go_further(1);
          on.push(Word::As(like));
        } else if word == "--as-load" {
          let load_name = self.get_arg_on();
          self.go_further(1);
          on.push(Word::AsLoad(load_name));
        } else if word == "--as-all-load" {
          on.push(Word::AsAllLoad);
        } else {
          panic!("Unknown dict word: {}", word);
        }
      } else {
        on.push(Word::As(word));
      }
    }
  }

  fn go_further(&mut self, by: usize) {
    self.index += by;
  }

  fn go_back(&mut self, by: usize) {
    self.index -= by;
  }

  fn get_arg_on(&self) -> String {
    if self.index < self.size() {
      self.args[self.index].clone()
    } else {
      String::default()
    }
  }

  // fn get_arg_by(&self, delta: usize) -> &str {
  //   let arg_by = self.index + delta;
  //   if arg_by >= self.size() {
  //     return "";
  //   }
  //   &self.args[arg_by]
  // }

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

fn is_closer(arg: &str) -> bool {
  arg == "from" || arg == "load" || arg == "save" || arg == ""
}
