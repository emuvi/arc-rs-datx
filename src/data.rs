use regex::Regex;

#[derive(Debug)]
pub struct From {
  pub name: String,
  pub kind: FromKind,
}

#[derive(Debug)]
pub enum FromKind {
  Path(String),
  StdInBody,
  StdInPath,
}

#[derive(Debug)]
pub struct Load {
  pub name: String,
  pub from: LoadFrom,
  pub hunt: Regex,
  pub zone: Zone,
}

#[derive(Debug)]
pub enum LoadFrom {
  All,
  Name(String),
}

#[derive(Debug)]
pub enum Zone {
  Body,
  Line,
  Load(String),
}

#[derive(Debug)]
pub enum Save {
  ToFile(OnFile),
}

#[derive(Debug)]
pub struct OnFile {
  pub path: Dict,
  pub body: Dict,
}

pub type Dict = Vec<Word>;

#[derive(Debug)]
pub enum Word {
  Like(String),
  Load(String),
}
