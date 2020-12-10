use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};

pub fn main() {
  if let Ok(file) = fs::File::open("input6.txt") {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

  }
}


