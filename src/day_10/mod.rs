use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

type SeatEntry = ((i32, i32), Seat);
type SeatMap = HashMap<(i32, i32), Seat>;

pub fn run() -> i32 {
  let file = File::open("src/day_10/input").unwrap();
  let mut seats: SeatMap = io::BufReader::new(file)
    .lines()
    .enumerate()
    .flat_map(|(line_index, line)| parse_line(line_index as i32, &line.unwrap()))
    .into_iter()
    .collect();

  while step(&mut seats) {}

  seats
    .values()
    .filter(|&&seat| seat == Seat::Occupied)
    .count() as i32
}

#[derive(Clone, Copy, PartialEq)]
enum Seat {
  Floor,
  Occupied,
  Empty,
}

fn parse_line(line_index: i32, line: &str) -> Vec<SeatEntry> {
  line
    .chars()
    .enumerate()
    .map(|(x, ch)| ((line_index, x as i32), parse_seat(ch)))
    .collect()
}

fn parse_seat(ch: char) -> Seat {
  match ch {
    'L' => Seat::Empty,
    '#' => Seat::Occupied,
    _ => Seat::Floor,
  }
}

fn step(seats: &mut SeatMap) -> bool {
  let changes = get_changes(seats);

  for &(key, value) in changes.iter() {
    seats.insert(key, value);
  }

  changes.len() > 0
}

fn get_changes(seats: &SeatMap) -> Vec<SeatEntry> {
  seats
    .iter()
    .flat_map(|(&key, seat)| {
      if seat != &Seat::Floor {
        let (x, y) = key;
        let active_neighbours = count_active_neighbours(seats, x, y);

        match seat {
          Seat::Empty if active_neighbours == 0 => Some((key, Seat::Occupied)),
          Seat::Occupied if active_neighbours >= 5 => Some((key, Seat::Empty)),
          _ => None,
        }
      } else {
        None
      }
    })
    .collect()
}

fn count_active_neighbours(seats: &SeatMap, x: i32, y: i32) -> i32 {
  let mut count = 0;
  for &offset_x in [-1, 0, 1].iter() {
    for &offset_y in [-1, 0, 1].iter() {
      if !(offset_x == 0 && offset_y == 0) {
        if find_occupied_neighbour(seats, x, y, offset_x, offset_y) {
          count += 1;
        }
      }
    }
  }

  count
}

fn find_occupied_neighbour(
  seats: &SeatMap,
  start_x: i32,
  start_y: i32,
  dir_x: i32,
  dir_y: i32,
) -> bool {
  let mut x = start_x;
  let mut y = start_y;
  loop {
    x += dir_x;
    y += dir_y;

    match seats.get(&(x, y)) {
      None => return false,
      Some(seat) => match seat {
        Seat::Occupied => return true,
        Seat::Empty => return false,
        _ => (),
      },
    }
  }
}
