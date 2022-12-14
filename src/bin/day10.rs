use std::str::FromStr;

use anyhow::Result;

enum Instr {
    Noop,
    Addr(i16),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            return Ok(Instr::Noop);
        } else {
            let (_, val) = s.split_once(' ').unwrap();
            let val = val.parse().unwrap();
            return Ok(Instr::Addr(val));
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("day10.input");
    let mut total = 0;
    let mut cycle = 1;
    let mut x = 1;
    let mut crt = "".to_string();

    input.lines()
        .map(|line| line.parse::<Instr>().unwrap())
        .for_each(|instr| {
            if cycle % 40 >= x && cycle % 40 <= x + 2 {
                crt += "#";
            } else {
                crt += " ";
            }
            
            cycle += 1;

            match instr {
                Instr::Addr(val) => {
                    if cycle % 40 == 20 {
                        total += cycle * x;
                    } else if (cycle % 40) == 1 {
                        crt += "\n";
                    }

                    if cycle % 40 >= x && cycle % 40 <= x + 2 {
                        crt += "#";
                    } else {
                        crt += " ";
                    }

                    cycle += 1;
                    x += val;
                },
                Instr::Noop => {},
            }

            if cycle % 40 == 20 {
                total += cycle * x;
            } else if (cycle % 40) == 1 {
                crt += "\n";
            }
        });
        
    println!("{}", total);
    println!("{}", crt);

    return Ok(());
}