use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

// create a function that takes a string and returns an RPS enum
fn parse_rps(s: &str) -> Result<RPS> {
    match s {
        // for "A"and "a" return RPS::Rock
        "A" | "X" => Ok(RPS::Rock),
        "B" | "Y" => Ok(RPS::Paper),
        "C" | "Z" => Ok(RPS::Scissors),
        _ => Err(anyhow!("Invalid input")),
    }
}

fn main() ->  Result<()> {
    let input = include_str!("day2.input");
    let mut score = 0;
    let mut my_strategy_score = 0;

    input.lines().for_each(|line| {
        // read first and third character
        let first = &line[0..1];
        let second = &line[2..3];
        let first_rpc = parse_rps(first).unwrap();
        let second_rpc = parse_rps(second).unwrap();
        
        score += match (first_rpc, second_rpc) {
            (RPS::Rock, RPS::Rock) => 1 + 3,
            (RPS::Rock, RPS::Paper) => 2 + 6,
            (RPS::Rock, RPS::Scissors) => 3 + 0,
            (RPS::Paper, RPS::Rock) => 1 + 0,
            (RPS::Paper, RPS::Paper) => 2 + 3,
            (RPS::Paper, RPS::Scissors) => 3 + 6,
            (RPS::Scissors, RPS::Rock) => 1 + 6,
            (RPS::Scissors, RPS::Paper) => 2 + 0,
            (RPS::Scissors, RPS::Scissors) => 3 + 3,
        };

        my_strategy_score += match (first_rpc, second) {
            (RPS::Rock, "X") => 3 + 0,
            (RPS::Rock, "Y") => 1 + 3,
            (RPS::Rock, "Z") => 2 + 6,
            (RPS::Paper, "X") => 1 + 0,
            (RPS::Paper, "Y") => 2 + 3,
            (RPS::Paper, "Z") => 3 + 6,
            (RPS::Scissors, "X") => 2 + 0,
            (RPS::Scissors, "Y") => 3 + 3,
            (RPS::Scissors, "Z") => 1 + 6,
            (_, _) => 0,
        }
    });

    println!("Score: {}", score);
    println!("My strategy score: {}", my_strategy_score);

    return Ok(());
}