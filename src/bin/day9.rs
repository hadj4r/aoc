use std::{str::FromStr, collections::HashSet};

use anyhow::Result;

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

fn get_delta(direction: &Direction) -> (i16, i16) {
    match direction {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn sign(x: i16) -> i16 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

fn main() -> Result<()> {
    let input = include_str!("day9.input");
    let mut rope = vec![(0, 0); 10];
    let mut tails = HashSet::new();
    tails.insert((0, 0));

    input.lines()
        .map(|s| s.parse::<Move>().unwrap())
        .for_each(|m| {
            for _ in 0..m.distance {
                let (hx, hy) = rope[0];
                let (dx, dy) = get_delta(&m.direction);
                rope[0] = (hx + dx, hy + dy);

                for i in 0..9 {
                    let (hx, hy) = rope[i];
                    let (tx, ty) = rope[i + 1];
                    let (dx, dy) = (hx - tx, hy - ty);
                    
                    if dx.pow(2) + dy.pow(2) > 2 {
                        rope[i + 1] = (tx + sign(dx), ty + sign(dy));
                    }
                }

                tails.insert(rope[9]);
            }
        });

        println!("tails: {}", tails.len());
        
        Ok(())
}