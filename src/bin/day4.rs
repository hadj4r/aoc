use anyhow::Result;

fn main() ->  Result<()> {
    // read file to string
    let input = include_str!("day4.input");

    let sum = input.lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| {
            let (first_start, first_end) = a.split_once("-").unwrap();
            let (second_start, second_end) = b.split_once("-").unwrap();
            let first_start = first_start.parse::<u32>().unwrap();
            let first_end = first_end.parse::<u32>().unwrap();
            let second_start = second_start.parse::<u32>().unwrap();
            let second_end = second_end.parse::<u32>().unwrap();
            // return 1 if from to is overlapping with from to
            // return 0 if not
            if first_start <= second_end && first_end >= second_end || second_start <= first_end && second_end >= first_end {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("{}", sum);
        
    return Ok(());
}