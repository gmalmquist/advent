use std::cmp;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};

fn main() {
  if let Ok(file) = fs::File::open("input5.txt") {
      let lines = io::BufReader::new(file).lines();
      let mut seats: Vec<Seat> = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap())
          .map(|l| Seat::new(&l))
          .collect();
      println!("Loaded {} seats.", seats.len());


      seats.sort_by(|a, b| if a.id() < b.id() { cmp::Ordering::Less } else if a.id() > b.id() { cmp::Ordering::Greater } else { cmp::Ordering::Equal });

      println!("Min: {}", seats[0]);
      println!("Max: {}", seats[seats.len() - 1]);

      for i in 1..(seats.len() - 1) {
          let a = seats[i - 1].id();
          let b = seats[i].id();
          let c = seats[i + 1].id();

          if c - a == 3 {
              // There should be two seats between a and c, but a number is missing.
              println!("ABC: {}, {}, {}", a, b, c);
              if a == b - 1 {
                  println!("  Missing seat is {}", b + 1);
              } else {
                  println!("  Missing seat is {}", b - 1);
              }
              break;
          }
      }
  }
}

#[derive(Debug)]
struct Seat {
  token: String,
  row: u32,
  col: u32,
}

impl Seat {
    fn new(token: &str) -> Self {
        let mut min_row = 0;
        let mut max_row = 127;
        let mut min_col = 0;
        let mut max_col = 7;
        for c in token.chars() {
            match c {
                'F' => {
                    max_row = (max_row + min_row) / 2;
                },
                'B' => {
                    min_row = (max_row + min_row) / 2 + 1;
                },
                'L' => {
                    max_col = (max_col + min_col) / 2;
                },
                'R' => {
                    min_col = (max_col + min_col) / 2 + 1;
                },
                _ => panic!("unknown letter."),
            }
        }
        assert!(min_row == max_row, "incoherent row");
        assert!(min_col == max_col, "incoherent col");
        Self {
            token: token.to_string(),
            row: min_row,
            col: min_col,
        }
    }

    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Seat({}, ({}, {}), {})", self.token, self.row, self.col, self.id())
    }
}
