extern crate itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn run() -> i32 {
  let file = File::open("day_2/input").unwrap();
  io::BufReader::new(file)
    .lines()
    .map(|line| Line::from_str(&line.unwrap()).unwrap())
    .filter(line_valid)
    .count() as i32
}

fn line_valid(line: &Line) -> bool {
  let count = line.password.matches(line.rule.letter).count() as i32;
  let bounds = &line.rule.bounds;

  count >= bounds.min && count <= bounds.max
}

struct Line {
  rule: Rule,
  password: String,
}

struct Rule {
  bounds: Bounds,
  letter: char,
}

struct Bounds {
  min: i32,
  max: i32,
}

impl FromStr for Line {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (rule, password) = split_at_char(input, ':');
    Ok(Line {
      rule: Rule::from_str(rule).unwrap(),
      password: password.to_string(),
    })
  }
}

impl FromStr for Rule {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (bounds, letter) = split_at_char(input, ' ');
    Ok(Rule {
      bounds: Bounds::from_str(bounds).unwrap(),
      letter: letter.chars().nth(1).unwrap(),
    })
  }
}

impl FromStr for Bounds {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (min, max) = split_at_char(input, '-');
    Ok(Bounds {
      min: i32::from_str_radix(min, 10).unwrap(),
      max: i32::from_str_radix(&max[1..], 10).unwrap(),
    })
  }
}

fn split_at_char(s: &str, ch: char) -> (&str, &str) {
  let index = s.find(ch).unwrap();
  s.split_at(index)
}
