use regex::Regex;

#[derive(Debug)]
pub struct Datx {
  pub from: Vec<From>,
  pub load: Vec<Load>,
  pub save: Vec<Save>,
}

#[derive(Debug)]
pub struct From {
  pub name: String,
  pub kind: FromKind,
}

#[derive(Debug)]
pub enum FromKind {
  Path(String),
  StdInPath,
  StdInBody,
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
  The(String),
}

#[derive(Debug)]
pub enum Zone {
  OnBody,
  OnLine,
  OnLoad(String),
}

#[derive(Debug)]
pub enum Save {
  ToFile(File),
}

#[derive(Debug)]
pub struct File {
  pub path: Dict,
  pub body: Dict,
}

pub type Dict = Vec<Word>;

#[derive(Debug)]
pub enum Word {
  Like(String),
  Load(String),
}
