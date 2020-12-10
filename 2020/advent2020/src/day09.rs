


use std::fs;
use std::io::{self, BufRead};

pub fn main() {
  if let Ok(file) = fs::File::open("input6.txt") {
      let lines = io::BufReader::new(file).lines();
      let _lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

  }
}


