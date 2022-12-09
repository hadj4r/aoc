use std::{str::FromStr, collections::HashSet};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: i16,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_once(' ').unwrap();
        let direction = direction.parse::<Direction>()?;
        let distance = distance.parse().unwrap();
        Ok(Move { direction, distance })
    }
}

fn main() -> Result<()> {
    let input = include_str!("day9.input");
    let mut unique_points: HashSet<(i16, i16)> = HashSet::new();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    unique_points.insert(tail);

    input.lines()
        .map(Move::from_str)
        .map(Result::unwrap)
        .for_each(|m| {
            // println!("== {:?} {} ==", m.direction, m.distance);
            head = match m.direction {
                Direction::Up => (head.0, head.1 + m.distance),
                Direction::Down => (head.0, head.1 - m.distance),
                Direction::Left => (head.0 - m.distance, head.1),
                Direction::Right => (head.0 + m.distance, head.1),
            };

            // distance between tail and head must not be greater than 1
            // if it is, we need to move the tail closer to the head
            // until the distance is 1
            while (tail.0 - head.0).abs() + (tail.1 - head.1).abs() > 1
                && (tail.0 - head.0).abs() != (tail.1 - head.1).abs()  {
                // print!("{:?}", tail);
                // move tail closer to head until distance is 1, tail can move in any direction as long as it's closer to head including diagonally
                tail = if tail.0 > head.0 {
                    if tail.1 > head.1 {
                        (tail.0 - 1, tail.1 - 1)
                    } else if tail.1 < head.1 {
                        (tail.0 - 1, tail.1 + 1)
                    } else {
                        (tail.0 - 1, tail.1)
                    }
                } else if tail.0 < head.0 {
                    if tail.1 > head.1 {
                        (tail.0 + 1, tail.1 - 1)
                    } else if tail.1 < head.1 {
                        (tail.0 + 1, tail.1 + 1)
                    } else {
                        (tail.0 + 1, tail.1)
                    }
                } else {
                    if tail.1 > head.1 {
                        (tail.0, tail.1 - 1)
                    } else if tail.1 < head.1 {
                        (tail.0, tail.1 + 1)
                    } else {
                        (tail.0, tail.1)
                    }
                };
                
                // println!(" -> {:?}", tail);
                unique_points.insert(tail);
            }
            // println!("head: {:?}", head);
        });
        println!("{:?}", unique_points.len());

    Ok(())
}