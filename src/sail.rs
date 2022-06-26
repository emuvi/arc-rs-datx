use regex::Regex;
use rubx::RubxResult;

use crate::ways::*;

impl Datx {
  pub fn start(self) -> RubxResult<()> {
    let mut retrieved: Vec<Crude> = Vec::new();
    for from in self.from {
      let data: Vec<String> = match from.kind {
        Kind::Path(path) => read_path(path)?,
        Kind::StdInPath => read_paths(read_stdin()?)?,
        Kind::StdInBody => read_stdin()?,
      };
      retrieved.push(Crude {
        name: from.name,
        data,
      });
    }
    println!("Retrieved: {:?}", retrieved);
    let mut processed: Vec<Cooked> = Vec::new();
    for pick in self.pick {
      let data: Vec<String> = cook(&retrieved, &processed, pick.hunt, pick.zone)?;
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
  data: Vec<String>,
}

/// Cooked is where the data that has been transformed is stored.
#[derive(Debug)]
struct Cooked {
  name: String,
  data: Vec<String>,
}

fn read_path(path: String) -> RubxResult<Vec<String>> {
  use std::io::BufRead;
  let file = std::fs::File::open(path)?;
  let mut results = Vec::new();
  for line in std::io::BufReader::new(file).lines() {
    results.push(line?);
  }
  Ok(results)
}

fn read_paths(paths: Vec<String>) -> RubxResult<Vec<String>> {
  use std::io::BufRead;
  let mut results = Vec::new();
  for path in paths {
    let file = std::fs::File::open(path)?;
    for line in std::io::BufReader::new(file).lines() {
      results.push(line?);
    }
  }
  Ok(results)
}

fn read_stdin() -> RubxResult<Vec<String>> {
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
  zone: Zone,
) -> RubxResult<Vec<String>> {
  let mut results = vec![];
  let collected = collect(retrieved, processed, zone);
  for line in collected {
    for found in hunt.find_iter(line) {
      results.push(found.as_str().into());
    }
  }
  Ok(results)
}

fn collect<'a>(
  retrieved: &'a Vec<Crude>,
  processed: &'a Vec<Cooked>,
  zone: Zone,
) -> Vec<&'a String> {
  let mut results: Vec<&String> = vec![];
  match zone {
    Zone::AllCrude => {
      for crude in retrieved {
        for line in &crude.data {
          results.push(line);
        }
      }
    }
    Zone::OnCrude(name) => {
      if let Some(crude) = retrieved.iter().find(|crude| crude.name == name) {
        for line in &crude.data {
          results.push(line);
        }
      }
    }
    Zone::OnCooked(name) => {
      if let Some(crude) = processed.iter().find(|crude| crude.name == name) {
        for line in &crude.data {
          results.push(line);
        }
      }
    }
  };
  results
}
