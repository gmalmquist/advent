use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

use lazy_static::lazy_static;
use regex::Regex;

// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6

lazy_static! {
static ref INST_PATTERN: Regex = Regex::new(r"^(?P<cmd>\w+)\s+([+]?)(?P<amt>[-]?\d+)$").unwrap();
}

pub fn main() {
  if let Ok(file) = fs::File::open("inputs/08.txt") {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

      let program: Vec<Instruction> = lines.map(|line| Instruction::parse(&line))
          .collect();
      println!("Loaded {} line program.", program.len());

      let mut executed: HashSet<usize> = HashSet::new();
      let mut fp = 0;
      let mut acc = 0;

      while fp < program.len() {
          if executed.contains(&fp) {
              println!("Halting before loop, acc = {}", acc);
              break;
          }
          executed.insert(fp);
          let Instruction { cmd, amt } = &program[fp];
          let cmd: &str = &cmd;
          match (cmd, amt) {
              ("nop", _) => {
                  fp += 1;
              },
              ("acc", amt) => {
                  acc += amt;
                  fp += 1;
              },
              ("jmp", amt) => {
                  fp = (fp as i32 + *amt) as usize;
              },
              unrecognized => {
                  println!("Unrecognized instruction: {:?}", unrecognized);
                  break;
              },
          }
      }
  }
}

#[derive(Debug)]
struct Instruction {
    cmd: String,
    amt: i32,
}

impl Instruction {
    fn parse(line: &str) -> Self {
      let m = INST_PATTERN.captures(&line).unwrap();
      let cmd = m.group("cmd", &line).unwrap().to_string();
      let amt = m.group("amt", &line).unwrap();
      let amt: i32 = amt.parse().unwrap();
      Self { cmd, amt }
    }
}

trait GetGroup {
  fn group<'a>(&self, name: &str, text: &'a str) -> Option<&'a str>;
}

impl GetGroup for regex::Captures<'_> {
  fn group<'a>(&self, name: &str, text: &'a str) -> Option<&'a str> {
      self.name(name)
          .map(|g| g.range())
          .map(|r| &text[r])
  }
}
