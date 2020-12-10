use std::fs;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

pub fn main() {
  if let Ok(file) = fs::File::open("input4.txt") {
      let lines = io::BufReader::new(file).lines();
      let mut valid_count = 0;
      let mut map = HashMap::new();
      for line in lines {
          if let Ok(line) = line {
              if line.len() == 0 {
                  valid_count += is_valid(&map) as usize;
                  map = HashMap::new();
              }
              parse_into(&line, &mut map);
          }
      }
      if map.len() > 0 {
          valid_count += is_valid(&map) as usize;
      }
      println!("Valid Passports: {}", valid_count);
  }
}

fn is_valid(pass: &HashMap<String, String>) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid = required.iter().map(|k| k.to_string()).filter(|k| pass.contains_key(k)).count() == required.len()
        && required.iter().fold(true, |b, k| b && pass.get(*k).map_or(false, |v| {
            let valid = is_field_valid(k, v);
            valid
        }));
    valid
}

fn is_field_valid(key: &str, val: &str) -> bool {
  let eyes: HashSet<String> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter()
      .map(|s| s.to_string())
      .collect();
  match (key, val) {
      ("byr", val) => check_num_range(val, 1920, 2002),
      ("iyr", val) => check_num_range(val, 2010, 2020),
      ("eyr", val) => check_num_range(val, 2020, 2030),
      ("hgt", val) => {
          match parse_unit(val) {
              (num, unit) if unit == "cm" => check_num_range(&num, 150, 193),
              (num, unit) if unit == "in" => check_num_range(&num, 59, 76),
              _ => false,
          }
      },
      ("hcl", val) => {
          val.chars().enumerate()
              .map(|(i, c)| (i == 0 && c == '#') || (i > 0 && ((c >= '0' && c <= '0') || (c >= 'a' && c <= 'f'))))
              .count() == 7
      },
      ("ecl", val) => eyes.contains(val),
      ("pid", val) => val.len() == 9 && val.chars().fold(true, |b, c| b && (c >= '0' && c <= '9')),
      ("cid", _) => true,
      _ => panic!("unrecognized field."),
  }
}

fn parse_unit(text: &str) -> (String, String) {
    let mut num = String::new();
    let mut unit = String::new();
    for c in text.chars() {
        if unit.len() > 0 || (c < '0' || c > '9') {
            unit.push(c);
            continue;
        }
        num.push(c);
    }
    (num, unit)
}

fn check_num_range(val: &str, min: u32, max: u32) -> bool {
    val.parse::<u32>().map_or(false, |num| num >= min && num <= max)
}

fn parse_into(line: &str, map: &mut HashMap<String, String>) {
    let mut state: ParseState = ParseState::Nada;
    let mut key = String::from("");
    let mut val = String::from("");

    for c in line.chars() {
        match (&state, &c) {
            (ParseState::Nada, ' ') => {},
            (ParseState::Nada, c) => {
                state = ParseState::InKey;
                key.push(*c);
            },
            (ParseState::InKey, ':') => {
                state = ParseState::InVal;
            },
            (ParseState::InKey, c) => {
                key.push(*c);
            },
            (ParseState::InVal, ' ') => {
                state = ParseState::Nada;
                map.insert(key.to_string(), val.to_string());
                key = String::from("");
                val = String::from("");
            },
            (ParseState::InVal, c) => {
                val.push(*c);
            }
        }
    }
    if val.len() > 0 {
        map.insert(key.to_string(), val.to_string());
    }
}

enum ParseState {
    Nada,
    InKey,
    InVal,
}

  //fn new(lines: std::io::Lines<io::BufReader<fs::File>>) -> Self {
