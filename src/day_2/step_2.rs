use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn run() -> i32 {
  let file = File::open("src/day_2/input").unwrap();
  io::BufReader::new(file)
    .lines()
    .map(|line| Line::from_str(&line.unwrap()).unwrap())
    .filter(line_valid)
    .count() as i32
}

fn line_valid(line: &Line) -> bool {
  let pos_matches = |pos| line.password.chars().nth(pos - 1).unwrap() == line.rule.letter;
  let positions = &line.rule.positions;
  pos_matches(positions.a) ^ pos_matches(positions.b)
}

struct Line {
  rule: Rule,
  password: String,
}

struct Rule {
  positions: Positions,
  letter: char,
}

struct Positions {
  a: usize,
  b: usize,
}

impl FromStr for Line {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (rule, password) = split_at_char(input, ':');
    Ok(Line {
      rule: Rule::from_str(rule).unwrap(),
      password: password[2..].to_string(),
    })
  }
}

impl FromStr for Rule {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (bounds, letter) = split_at_char(input, ' ');
    Ok(Rule {
      positions: Positions::from_str(bounds).unwrap(),
      letter: letter.chars().nth(1).unwrap(),
    })
  }
}

impl FromStr for Positions {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (min, max) = split_at_char(input, '-');
    Ok(Positions {
      a: i32::from_str_radix(min, 10).unwrap() as usize,
      b: i32::from_str_radix(&max[1..], 10).unwrap() as usize,
    })
  }
}

fn split_at_char(s: &str, ch: char) -> (&str, &str) {
  let index = s.find(ch).unwrap();
  s.split_at(index)
}
