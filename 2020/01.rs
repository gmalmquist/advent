use std::fs;
use std::io::{self, BufRead};

fn main() {
  if let Ok(file) = fs::File::open("input1.txt") {
      let mut numbers = vec![];
      let lines = io::BufReader::new(file).lines();
      for line in lines {
          if let Ok(num) = line {
              if num.len() == 0 {
                  continue;
              }
              let num: u32 = num.to_string().parse()
                  .expect("Not a number.");
              numbers.push(num);
          }
      }

      for i in 0..numbers.len() {
          let a = numbers[i];
          for j in (i+1)..numbers.len() {
              let b = numbers[j];
              if a + b == 2020 {
                  println!("A: {}, B: {}, *: {}",
                           a, b, a*b);
              }
          }
      }

      for i in 0..numbers.len() {
          let a = numbers[i];
          if a > 2020 {
              continue;
          }
          for j in (i+1)..numbers.len() {
              let b = numbers[j];
              if a + b > 2020 {
                  continue;
              }
              for k in (j+1)..numbers.len() {
                  let c = numbers[k];
                  if a + b + c== 2020 {
                      println!("A: {}, B: {}, C: {}, *: {}",
                               a, b, c, a*b*c);
                  }
              }
          }
      }
  }

}
