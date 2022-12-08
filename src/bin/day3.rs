// use nightly
#![feature(iter_array_chunks)]

use anyhow::Result;

fn main() ->  Result<()> {
    let input = include_str!("day3.input");

    // create a 2d array counting the number of each lowercase and uppercase letter 3 times
    let mut counts = [[0; 26 * 2]; 3];

    let mut sum = 0;

    // split the input by 3 lines at a time
    input.lines()
        .array_chunks::<3>()
        // iterate over each group
        .for_each(|group| {
            // iterate over each line in the group
            group.iter().enumerate().for_each(|(i, line)| {
                // iterate over each character in the line
                line.chars().for_each(|c| {
                    // convert the character to an index
                    let index = match c {
                        'a'..='z' => c as usize - 'a' as usize,
                        'A'..='Z' => c as usize - 'A' as usize + 26,
                        _ => return,
                    };

                    // increment the count for the character
                    counts[i][index] += 1;
                });

            });

            // find index of non zero value in all 3 arrays
            for i in 0..52 {
                if counts[0][i] > 0 && counts[1][i] > 0 && counts[2][i] > 0 {
                    sum += i + 1;
                    break;
                }
            }
           
            // reset counts for the next lines
            counts = [[0; 26 * 2]; 3];
        });


    println!("sum: {}", sum);

    return Ok(());
}