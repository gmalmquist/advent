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

      let mut sum_counts = 0;
      let mut group: HashMap<char, u32> = HashMap::new();
      let mut group_size: u32 = 0;
      for line in lines {
          if line.len() == 0 {
              if group_size > 0 {
                  let counts = group.values()
                      .filter(|v| **v == group_size)
                      .count();
                  sum_counts += counts;
              }
              group_size = 0;
              group = HashMap::new();
              continue;
          }
          for c in line.chars() {
              group.insert(c, group.get(&c).unwrap_or(&0) + 1);
          }
          group_size += 1;
      }
      if group_size > 0 {
          sum_counts += group.values()
              .filter(|v| **v == group_size)
              .count();
      }

      println!("Sum: {}", sum_counts);
  }
}
