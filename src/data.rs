use regex::Regex;

pub enum Zone {
    Body,
    Line,
    Load(String),
}

pub struct Load {
    pub name: String,
    pub hunt: Regex,
    pub zone: Zone,
}

pub enum Word {
    Like(String),
    Load(String),
}

pub type Dict = Vec<Word>;

pub enum Save {
    ToFile(OnFile),
}

pub struct OnFile {
    pub path: Dict,
    pub body: Dict,
}
