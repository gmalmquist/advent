use std::fs;
use std::io::{self, BufRead};

fn main() {
  if let Ok(file) = fs::File::open("input3.txt") {
      let lines = io::BufReader::new(file).lines();
      let forest = Forest::new(lines);
      let slopes = vec![
          (1, 1),
          (3, 1),
          (5, 1),
          (7, 1),
          (1, 2),
      ];
      let mut product: u64 = 1;
      for (right, down) in slopes {
          let trees = forest.count_trees(right, down);
          product *= trees as u64;
          println!("slope {:?}, trees: {}, *: {}", (right, down), trees, product);
      }
  }
}

struct Forest {
  trees: Vec<Vec<char>>,
}

impl Forest {
  fn new(lines: std::io::Lines<io::BufReader<fs::File>>) -> Self {
      Self {
          trees: lines
              .filter(|l| l.is_ok())
              .map(|l| l.unwrap())
              .map(|l| l.chars().collect())
              .collect(),
      }
  }

  fn getc(&self, i: usize, j: usize) -> char {
      self.trees[i][j % self.trees[i].len()]
  }

  fn is_tree(&self, i: usize, j: usize) -> bool {
      self.getc(i, j) == '#'
  }

  fn count_trees(&self, right: usize, down: usize) -> u32 {
      let mut count = 0;
      let mut j = 0;
      for i in (0..self.trees.len()).step_by(down) {
          count += self.is_tree(i, j) as u32;
          j += right;
      }
      count
  }
}
