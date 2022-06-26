use regex::Regex;
use rubx::RubxResult;

use crate::ways::*;

impl Datx {
  pub fn start(self) -> RubxResult<()> {
    let mut retrieved: Vec<Crude> = Vec::new();
    for from in self.from {
      let data = match from.kind {
        Kind::Path(path) => read_path(path)?,
        Kind::StdInBody => read_stdin_body()?,
        Kind::StdInAllPaths => read_paths(read_stdin_lines()?)?,
        Kind::StdInLinePath => read_path(read_stdin_line()?)?,
      };
      retrieved.push(Crude {
        name: from.name,
        data,
      });
    }
    println!("Retrieved: {:?}", retrieved);
    let mut processed: Vec<Cooked> = Vec::new();
    for pick in self.pick {
      let data = cook(&retrieved, &processed, pick.hunt, pick.look, pick.zone)?;
      processed.push(Cooked {
        name: pick.name,
        data,
      });
    }
    println!("Processed: {:?}", processed);
    Ok(())
  }
}

/// Crude is where the data that comes From is stored without any transformation.
#[derive(Debug)]
struct Crude {
  name: String,
  data: String,
}

/// Cooked is where the data that has been transformed is stored.
#[derive(Debug)]
struct Cooked {
  name: String,
  data: String,
}

fn read_path(path: String) -> RubxResult<String> {
  Ok(std::fs::read_to_string(path)?)
}

fn read_paths(paths: Vec<String>) -> RubxResult<String> {
  let mut results = String::new();
  for path in paths {
    let file = std::fs::read_to_string(path)?;
    results.push_str(&file);
    results.push('\n');
  }
  Ok(results)
}

fn read_stdin_body() -> RubxResult<String> {
  use std::io::BufRead;
  let mut results = String::new();
  for line in std::io::stdin().lock().lines() {
    results.push_str(line?.as_str());
    results.push('\n');
  }
  Ok(results)
}

fn read_stdin_line() -> RubxResult<String> {
  use std::io::BufRead;
  for line in std::io::stdin().lock().lines() {
    return Ok(line?.into());
  }
  Ok(String::default())
}

fn read_stdin_lines() -> RubxResult<Vec<String>> {
  use std::io::BufRead;
  let mut results = Vec::new();
  for line in std::io::stdin().lock().lines() {
    results.push(line?.into());
  }
  Ok(results)
}

fn cook<'a>(
  retrieved: &'a Vec<Crude>,
  processed: &'a Vec<Cooked>,
  hunt: Regex,
  look: Look,
  zone: Zone,
) -> RubxResult<String> {
  let mut results = String::new();
  let collected = collect(retrieved, processed, look, zone);
  for line in collected {
    for found in hunt.find_iter(line) {
      results.push_str(found.as_str());
      results.push('\n');
    }
  }
  Ok(results)
}

fn collect<'a>(
  retrieved: &'a Vec<Crude>,
  processed: &'a Vec<Cooked>,
  look: Look,
  zone: Zone,
) -> Vec<&'a str> {
  let mut results: Vec<&str> = vec![];
  match zone {
    Zone::OnAllCrude => {
      for crude in retrieved {
        match look {
          Look::OnWhole => results.push(crude.data.as_str()),
          Look::OnLines => {
            for line in crude.data.lines() {
              results.push(line);
            }
          }
        }
      }
    }
    Zone::OnAllCooked => {
      for cooked in processed {
        match look {
          Look::OnWhole => results.push(cooked.data.as_str()),
          Look::OnLines => {
            for line in cooked.data.lines() {
              results.push(line);
            }
          }
        }
      }
    }
    Zone::OnCrude(name) => {
      if let Some(crude) = retrieved.iter().find(|crude| crude.name == name) {
        match look {
          Look::OnWhole => results.push(crude.data.as_str()),
          Look::OnLines => {
            for line in crude.data.lines() {
              results.push(line);
            }
          }
        }
      }
    }
    Zone::OnCooked(name) => {
      if let Some(cooked) = processed.iter().find(|cooked| cooked.name == name) {
        match look {
          Look::OnWhole => results.push(cooked.data.as_str()),
          Look::OnLines => {
            for line in cooked.data.lines() {
              results.push(line);
            }
          }
        }
      }
    }
  };
  results
}
