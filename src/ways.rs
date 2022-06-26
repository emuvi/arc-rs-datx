use regex::Regex;

#[derive(Debug)]
pub struct Datx {
  pub verbose: bool,
  pub from: Vec<From>,
  pub pick: Vec<Pick>,
  pub save: Vec<Save>,
}

#[derive(Debug)]
pub struct From {
  pub name: String,
  pub kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
  Path(String),
  StdInBody,
  StdInAllPaths,
  StdInLinePath,
}

#[derive(Debug)]
pub struct Pick {
  pub name: String,
  pub hunt: Regex,
  pub look: Look,
  pub zone: Zone,
}

#[derive(Debug)]
pub enum Look {
  OnWhole,
  OnLines,
}

#[derive(Debug)]
pub enum Zone {
  OnAllCrude,
  OnAllCooked,
  OnCrude(String),
  OnCooked(String),
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
  As(String),
  AsPicked(String),
  AsAllPicked,
}
