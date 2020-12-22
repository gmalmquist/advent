
use std::fs;
use std::io::{self, BufRead};


pub fn main() {
  let invalid_number = find_invalid_number(25);

  if let Ok(file) = fs::File::open("inputs/09.txt") {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

      let mut sum = RunningSum::new();

      for (_index, line) in lines.enumerate() {
          let num: u64 = line.parse().expect("couldn't parse input number");
          sum.push(num);

          if sum.sum == invalid_number && sum.length > 1 {
              println!("Encryption weakness: {}", get_weakness(&sum));
          }

          if sum.sum > invalid_number {
              // This conditional feels like a hack. It's correct, but
              // just doesn't seem very clean.
              let mut total = 0;
              for i in 0..sum.length {
                  total += sum.get(i);
                  if total == invalid_number {
                      sum.length = i + 1;
                      println!("Encryption weakness: {}", get_weakness(&sum));
                      return;
                  }
              }
          }

          while sum.sum > invalid_number {
              sum.remove_start();
          }
      }
      println!("Did not find an encryption weakness.");
  }
}

fn get_weakness(sum: &RunningSum) -> u64 {
  let mut min: Option<u64> = None;
  let mut max: Option<u64> = None;
  for i in 0..sum.length {
      let num = sum.get(i);
      min = Some(min.map_or(num, |m| if num < m { num } else { m }));
      max = Some(max.map_or(num, |m| if num > m { num } else { m }));
  }
  let min = min.unwrap();
  let max = max.unwrap();
  println!("Sum of {} + {} = {}", min, max, min + max);
  min + max
}

fn find_invalid_number(preamble_size: usize) -> u64 {
  if let Ok(file) = fs::File::open("inputs/09.txt") {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

      let mut buffer = RingBuffer::new(preamble_size);

      for (index, line) in lines.enumerate() {
          let num: u64 = line.parse().expect("couldn't parse input number");
          if index < preamble_size {
              buffer.push(num);
              continue;
          }

          if !buffer.has_addends(num) {
              println!("Number does not have addends: {} (line {})", num, index);
              return num;
          }

          buffer.push(num);
      }
  }
  panic!("Didn't find number without addends.");
}

struct RunningSum {
    store: Vec<u64>,
    start: usize,
    length: usize,
    sum: u64,
}

impl RunningSum {
    fn new() -> Self {
        Self {
            store: vec![],
            start: 0,
            length: 0,
            sum: 0,
        }
    }

    fn push(&mut self, num: u64) {
        self.sum += num;

        let capacity = self.store.len();
        let index = self.start + self.length;
        if index < capacity {
            self.store[index] = num;
            self.length += 1;
            return;
        }
        if index - capacity < self.start {
            self.store[index - capacity] = num;
            self.length += 1;
            return;
        }
        if self.start > 0 {
            self.recenter();
        }
        self.store.push(num);
        self.length += 1;
    }

    fn remove_start(&mut self) {
        if self.length == 0 {
            return;
        }
        self.sum -= self.store[self.start];
        self.start = (self.start + 1) % self.store.len();
        self.length -= 1;
    }

    fn recenter(&mut self) {
        let mut vec = vec![];
        for i in 0..self.length {
            vec.push(self.store[(self.start + i) % self.store.len()]);
        }
        self.store = vec;
    }

    fn get(&self, i: usize) -> u64 {
        self.store[(self.start + i) % self.store.len()]
    }

    //fn values(&self) -> Vec<u64> {
    //    let mut vec = vec![];
    //    for i in 0..self.length {
    //        vec.push(self.get(i));
    //    }
    //    vec
    //}
}

struct RingBuffer<T> {
    capacity: usize,
    cursor: usize,
    vec: Vec<T>,
}

impl<T> RingBuffer<T> {
  fn new(capacity: usize) -> Self {
      Self {
          capacity,
          cursor: 0,
          vec: vec![],
      }
  }

  //fn remove_first(&mut self) {
  //    self.cursor += 1;
  //}

  fn push(&mut self, item: T) {
      let index = self.cursor % self.capacity;
      if index >= self.vec.len() {
          self.vec.push(item);
      } else {
          self.vec[index]= item;
      }
      self.cursor += 1;
  }

  fn get(&self, index: isize) -> &T {
      let index = (self.cursor as isize + index + self.capacity as isize) as usize 
          % self.capacity;
      &self.vec[index]
  }
}

trait HasAddends {
  fn has_addends(&self, num: u64) -> bool;
}

impl HasAddends for RingBuffer<u64> {
    fn has_addends(&self, num: u64) -> bool {
      for ai in (-(self.capacity as isize))..(-1) {
          let a = *self.get(self.cursor as isize + ai);
          for bi in (ai+1)..(self.capacity as isize) {
              let b = *self.get(bi);
              if a + b == num {
                  return true;
              }
          }
      }
      false
        
    }
}
