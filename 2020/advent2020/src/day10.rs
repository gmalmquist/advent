use std::collections::{HashMap};
use std::env;
use std::fs;
use std::io::{self, BufRead};


pub fn main() {
  if let Ok(file) = fs::File::open(format!("inputs/{}.txt", env::args().nth(1).unwrap())) {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

      let mut adapters: Vec<u32> = lines.map(|l| l.parse::<u32>().unwrap())
          .collect();

      println!("Loaded {} adapters.", adapters.len());

      adapters.sort();

      // built-in device adapter
      adapters.push(adapters[adapters.len() - 1] + 3);

      let mut differences: HashMap<u32, u32> = HashMap::new();
      let mut last = 0;
      for adapter in adapters.iter() {
          let diff = adapter - last;
          last = *adapter;
          if diff > 3 {
              panic!("Difference > 3 detected: {}", diff);
          }
          differences.insert(diff, differences.get(&diff).unwrap_or(&0) + 1);
      }

      println!("Using all adapters:");
      println!("  1jd * 3jd = {}",
               differences.get(&1).unwrap() * differences.get(&3).unwrap());

      let mut ways_to_get: HashMap<u32, u64> = HashMap::new();

      for (index, adapter) in adapters.iter().enumerate() {
          if index == 0 {
              ways_to_get.insert(*adapter, 1);
              continue;
          }
          let min = *vec![0, index as isize - 3].iter().max().unwrap() as usize;
          let mut ways = 0;
          if *adapter <= 3 {
              // We can start with this adapter.
              ways += 1;
          }

          for i in min..index {
              if adapter - adapters[i] <= 3 {
                  ways += ways_to_get.get(&adapters[i]).unwrap();
              }
          }
          ways_to_get.insert(*adapter, ways);
      }
      println!("Arrangements: {}",
               ways_to_get.get(&adapters[adapters.len() - 1]).unwrap());
  }
}

// Started to implement a graph to DFS this, but then realized this problem
// is much easier than that.
/*
struct Graph {
    vertices: HashSet<u32>,
    adjacency: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            adjacency: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, v: u32) {
        self.vertices.insert(v);
    }

    fn add_edge(&mut self, a: u32, b: u32) {
        self.add_vertex(a);
        self.add_vertex(b);
        if !self.adjacency.contains_key(&a) {
            self.adjacency.insert(a, vec![]);
        }
        let mut edges = self.adjacency.get_mut(&a).unwrap();
        edges.push(b);
    }

    fn dfs<F>(&self, start: u32, end: u32, visit: F) 
        where F: Fn(&Vec<u32>) {
    }
}
*/
