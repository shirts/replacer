use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

use dirs;
use serde::{Serialize, Deserialize};

const FILENAME: &'static str = ".replacer.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replacement {
  pub from: String,
  pub to: String,
  pub matches: String
}

impl Replacement {
  pub fn clear(&mut self) {
    self.matches.clear();
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Replacements {
  replacements: Vec<Replacement>
}

impl Replacements {
  pub fn write_sample_file() {
    let sample_replacements = "{
  \"replacements\":[
    {
      \"from\":\"foo\",\"to\":\"bar\",\"matches\":\"\"
    }
  ]
}";

    let file_path = Self::file_path();

    if let Ok(mut file) = fs::File::create(&file_path) {
      if let Err(err) = file.write_all(sample_replacements.as_bytes()) {
        eprintln!("There was an error when creating ({:?}): {}", &file_path, err);
      } else {
        println!("Wrote replacements to file: {:?}",  &file_path);
      }
    }
  }

  fn home_dir() -> String {
    dirs::home_dir().unwrap().into_os_string().into_string().unwrap()
  }

  pub fn file_path() -> PathBuf {
    Path::new(&Self::home_dir()).join(FILENAME)
  }

  pub fn filename() -> String {
    format!("{}/{}", Self::home_dir(), FILENAME)
  }

  pub fn load() -> Vec<Replacement> {

    let contents = fs::read_to_string(Self::filename());
    println!("filename: {}", Self::filename());

    match contents {
      Ok(contents) => {
        return Self::from_json_string(&contents).unwrap().replacements
      },
      Err(err) => {
        panic!("{}", err)
      }

    }
  }

  pub fn to_json_string(&self) -> Option<String> {
    match serde_json::to_string(&self) {
      Ok(string) => Some(string),
      Err(err) => {
        println!("Error converting replacements to_json_string. Error: {:?}", err);
        None
      }
    }
  }

  pub fn from_json_string(string: &str) -> Option<Replacements> {
    match serde_json::from_str(string) {
      Ok(replacements) => Some(replacements),
      Err(err) => {
        println!("Error reading replacements. Maybe a trailing comma? {}", err);
        None
      }
    }
  }
}
