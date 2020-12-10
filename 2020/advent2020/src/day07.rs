use std::collections::{HashMap, HashSet};
use std::cmp;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};

pub fn main() {
  if let Ok(file) = fs::File::open("input7.txt") {
      let lines = io::BufReader::new(file).lines();
      let lines = lines.filter(|l| l.is_ok())
          .map(|l| l.unwrap());

      let mut containers = HashMap::new();
      for line in lines {
          let rule = Rule::parse(&line);
          containers.insert(rule.parent.to_string(), rule);
      }
      let rules = RuleTree::new(containers);

      let mut count = 0;
      for bag in rules.bags() {
          if rules.can_contain_indirectly(&bag, "shiny gold") {
              count += 1;
          }
      }
      println!("Bags that can contain shiny gold: {}", count);

      let closure = rules.bag_closure("shiny gold");
      println!("Shiny gold bag closure: {:?}", closure);
      println!("Shiny gold bags must contain {} other bags.",
               closure.values().fold(0, |a, b| a + b));
  }
}

#[derive(Debug)]
struct RuleTree {
  rules: HashMap<String, Rule>,
}

impl RuleTree {
  fn new(rules: HashMap<String, Rule>) -> Self {
      Self { rules } 
  }

  fn bags(&self) -> Vec<String> {
      let mut vec: Vec<String> = self.rules.keys().map(|s| s.to_string()).collect();
      vec.sort();
      vec
  }

  fn can_contain_directly(&self, a: &str, b: &str) -> bool {
      self.rules.get(a).map_or(false, |a| a.children.contains_key(b))
  }

  fn bag_closure(&self, bag: &str) -> HashMap<String, u64> {
      let mut results = HashMap::new();
      let bag = self.rules.get(bag);
      if bag.is_none() {
          return results;
      }
      let bag = bag.unwrap();
      let mut frontier: Vec<(String, u64)> = bag.children.keys()
          .map(|s| (s.to_string(), *bag.children.get(s).unwrap() as u64))
          .collect();
      while !frontier.is_empty() {
          let (vertex, count) = frontier.pop().unwrap();
          results.insert(vertex.to_string(), 
                         results.get(&vertex).unwrap_or(&0) + count);
          let rule = self.rules.get(&vertex);
          if rule.is_none() {
              continue;
          }
          let rule = rule.unwrap();
          for child in rule.children.keys() {
              let child_count = *rule.children.get(child).unwrap() as u64;
              frontier.push((child.to_string(), count * child_count));
          }
      }
      results
  }

  fn can_contain_indirectly(&self, a: &str, b: &str) -> bool {
      self.bag_closure(a).contains_key(b)
  }
}

#[derive(Debug)]
struct Rule {
  parent: String,
  children: HashMap<String, u32>,
}

impl Rule {
    pub fn parse(text: &str) -> Self {
        let (parent, index) = parse_bag(text, 0);
        //println!("\n{}", text);
        //println!("  {} ({})", parent, index);
        assert!("contain" == &text[index..("contain".len()+index)]);

        let mut index = index + "contain ".len();
        let mut children: HashMap<String, u32> = HashMap::new();
        loop {
            match parse_num(text, index) {
                None => break,
                Some((num, i)) => {
                    //println!(" ...: '{}'", &text[i+1..]);
                    let (child, i) = parse_bag(text, i + 1);
                    index = i + 1;
                    children.insert(child, num);
                }
            }
        }

        Self {
            parent,
            children,
        }
    }
}

fn parse_num(text: &str, index: usize) -> Option<(u32, usize)> {
    let mut num = String::new();
    let mut end_index = index;
    for i in index..text.len() {
        let c = text.chars().nth(i).unwrap();
        if c < '0' || c > '9' {
            break;
        }
        num.push(c);
        end_index = i + 1;
    }
    let num: Result<u32, _> = num.parse();
    if num.is_err() {
        return None;
    }
    let num = num.unwrap();
    Some((num, end_index))
}

fn parse_bag(text: &str, index: usize) -> (String, usize) {
  let mut bag = String::new();
  let mut token = String::new();
  for i in index..text.len() {
      let c = text.chars().nth(i);
      if c.is_none() {
          break;
      }
      let c = c.unwrap();

      match c {
          ' ' | '.' | ',' => {
              if token == "bag" || token == "bags" {
                  return (bag, i + 1);
              }
              if bag.len() > 0 {
                  bag.push(' ');
              }
              bag.push_str(&token);
              token = String::new();
          },
          c => {
              token.push(c);
          }
      }
  }
  (bag, index)
}


