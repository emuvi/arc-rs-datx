use regex::Regex;

#[derive(Debug)]
pub enum From {
  Path(String),
  StdInBody,
  StdInPath,
}

#[derive(Debug)]
pub enum Zone {
  Body,
  Line,
  Load(String),
}

#[derive(Debug)]
pub struct Load {
  pub name: String,
  pub hunt: Regex,
  pub zone: Zone,
}

#[derive(Debug)]
pub enum Word {
  Like(String),
  Load(String),
}

pub type Dict = Vec<Word>;

#[derive(Debug)]
pub enum Save {
  ToFile(OnFile),
}

#[derive(Debug)]
pub struct OnFile {
  pub path: Dict,
  pub body: Dict,
}
