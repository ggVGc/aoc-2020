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
  Occupied,
  Empty,
}

fn parse_line(line_index: i32, line: &str) -> Vec<SeatEntry> {
  line
    .chars()
    .enumerate()
    .flat_map(|(x, ch)| parse_seat(ch).map(|seat| ((x as i32, line_index), seat)))
    .collect()
}

fn parse_seat(ch: char) -> Option<Seat> {
  match ch {
    'L' => Some(Seat::Empty),
    '#' => Some(Seat::Occupied),
    _ => None,
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
      let (x, y) = key;
      let active_neighbours = count_active_neighbours(seats, x, y);

      match seat {
        Seat::Empty if active_neighbours == 0 => Some((key, Seat::Occupied)),
        Seat::Occupied if active_neighbours >= 4 => Some((key, Seat::Empty)),
        _ => None,
      }
    })
    .collect()
}

fn count_active_neighbours(seats: &SeatMap, x: i32, y: i32) -> i32 {
  let mut count = 0;
  for &offset_x in [-1, 0, 1].iter() {
    for &offset_y in [-1, 0, 1].iter() {
      if !(offset_x == 0 && offset_y == 0) {
        match seats.get(&(x + offset_x, y + offset_y)) {
          None => (),
          Some(&neighbour) => {
            if neighbour == Seat::Occupied {
              count += 1;
            }
          }
        }
      }
    }
  }

  count
}
