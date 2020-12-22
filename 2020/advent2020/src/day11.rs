
use std::fs;
use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::fmt;
use std::collections::HashMap;


pub fn main() {
  if let Ok(file) = fs::File::open("inputs/11.txt") {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

      let grid = Grid::new(lines.collect());

      print!("Grid:\n{}\n", grid);
  }
}

#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
  fn new(lines: Vec<String>) -> Self {
      let rows = lines.len();
      let mut cols = 0;
      let mut chars = vec![];
      for line in lines {
          if line.len() > cols {
              cols = line.len();
          }
          line.chars().for_each(|c| chars.push(c));
      }
      Self {
          cells: chars,
          rows,
          cols,
      }
  }

  fn set(&mut self, pos: (usize, usize), val: char) {
      let (row, col) = pos;
    assert!(row < self.rows, "Row out of bounds {} < {}", row, self.rows);
    assert!(col < self.cols, "Col out of bounds {} < {}", col, self.cols);
    self.cells[row * self.cols + col] = val;
  }

  fn advance(&mut self) {
      // 1. If a seat is empty (L) and there are no occupied seats adjacent to it, the seat
      //    becomes occupied.
      // 2. If a seat is occupied (#) and four or more seats adjacent to it are also occupied,
      //    the seat becomes empty.
      // 3. Otherwise, the seat's state does not change.
      let mut change_map: HashMap<(usize,usize),char> = HashMap::new();
      for row in 0..self.rows {
          for col in 0..self.cols {
              if self[(row, col)] != 'L' {
                  continue;
              }
              let adj_acc = self.adjacent(row, col)
                  .filter(|c| *c == '#')
                  .count();

              let pos = (row, col);

              if adj_acc == 0 {
                  change_map.insert(pos, '#');
              } else if adj_acc >= 4 {
                  change_map.insert(pos, '#');
              }
          }
      }
      for (key, val) in change_map.iter() {
          self.set(*key, *val);
      }
  }

  fn adjacent<'a>(&'a self, row: usize, col: usize) -> Adj8<'a> {
      Adj8::new(&self, row, col)
  }
}

struct Adj8<'a> {
    grid: &'a Grid,
    positions: Vec<(usize, usize)>,
    i: usize,
}

impl<'a> Adj8<'a> {
    fn new(grid: &'a Grid, row: usize, col: usize) -> Self {
          let offsets: Vec<(isize, isize)> = vec![
              (-1, -1),
              (-1, 0),
              (-1, 1),
              (0, -1),
              (0, 1),
              (1, -1),
              (1, 0),
              (1, 1),
          ];
          let positions = offsets.iter()
              .map(|(r,c)| (r + row as isize, c + col as isize))
              .filter(|(r,c)| r > &0 && c > &0 && (*r as usize) < grid.rows && (*c as usize) < grid.cols)
              .map(|(r,c)| (r as usize, c as usize))
              .collect();
        Self {
            grid,
            positions,
            i: 0,
        }
    }
}

impl<'a> Iterator for Adj8<'a> {
  type Item = char;

  fn next(&mut self) -> Option<char> {
      if self.i >= self.positions.len() {
          return None;
      }
      let c = self.grid[self.positions[self.i]];
      self.i += 1;
      Some(c)
  }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut text = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                text.push(self[(row, col)]);
            }
            text.push('\n');
        }
        write!(f, "{}", text)
    }
}

impl Index<(usize,usize)> for Grid {
    type Output = char;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        let (row, col) = pos;
        assert!(row < self.rows, "Row out of bounds {} < {}", row, self.rows);
        assert!(col < self.cols, "Col out of bounds {} < {}", col, self.cols);
        &self.cells[row * self.cols + col]
    }
}

