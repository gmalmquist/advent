use std::fs;
use std::io::{self, BufRead};

fn main() {
  let mut valid = 0;
  if let Ok(file) = fs::File::open("input2.txt") {
      let lines = io::BufReader::new(file).lines();
      for line in lines {
          if let Ok(line) = line {
              if line.len() == 0 {
                  continue;
              }
              valid += check(&line) as u32;
          }
      }
  }
  print!("valid: {}", valid);
}

fn check(line: &str) -> bool {
    let dash = line.find("-").expect("no dash");
    let colon = line.find(":").expect("no colon");

    let one: usize = line[..dash].parse().expect("cannot parse min");
    let two: usize = line[dash+1..colon-2].parse().expect("cannot parse max");
    let chr = line.chars().nth(colon - 1).expect("no rule char");
    let pass = &line[colon+2..];
    pass.chars().enumerate().filter(|(i, c)| (*i == one - 1 || *i == two - 1) && c == &chr).count() == 1
}
