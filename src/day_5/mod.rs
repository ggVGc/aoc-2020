extern crate itertools;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run() -> i32 {
  let file = File::open("src/day_5/input").unwrap();

  let mut all_seat_ids: Vec<i32> = io::BufReader::new(file)
    .lines()
    .map(|line| parse_seat(&line.unwrap()))
    .map(|seat| get_seat_id(seat))
    .collect();

  let first_seat = *all_seat_ids.iter().min().unwrap();

  all_seat_ids.sort();

  let missing_seat = all_seat_ids
    .iter()
    .zip(all_seat_ids.iter().skip(1))
    .zip(all_seat_ids.iter().skip(2))
    .find_map(|((&a, &b), &c)| {
      if a > first_seat && b == a + 2 && c == b + 1 {
        Some(a + 1)
      } else {
        None
      }
    })
    .unwrap();

  println!("Missing seat: {}", missing_seat);

  *all_seat_ids.iter().max().unwrap()
}

struct Seat {
  row: i32,
  column: i32,
}

fn parse_seat(path: &str) -> Seat {
  let (row_path, column_path) = path.split_at(7);

  Seat {
    row: parse_seat_row(row_path),
    column: parse_seat_column(column_path),
  }
}

fn parse_seat_row(path: &str) -> i32 {
  parse_path(path, 'F', 'B', 0, 127)
}

fn parse_seat_column(path: &str) -> i32 {
  parse_path(path, 'L', 'R', 0, 7)
}

fn parse_path(
  path: &str,
  down_char: char,
  up_char: char,
  initial_min: i32,
  initial_max: i32,
) -> i32 {
  let mut min = initial_min;
  let mut max = initial_max;

  for ch in path.chars() {
    let mid = (max - min) / 2 + min;
    if ch == down_char {
      max = mid;
    } else if ch == up_char {
      min = mid + 1;
    } else {
      panic!("Invalid path")
    }
  }
  assert_eq!(min, max);

  min
}

fn get_seat_id(seat: Seat) -> i32 {
  seat.row * 8 + seat.column
}
