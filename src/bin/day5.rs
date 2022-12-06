use std::collections::VecDeque;

use anyhow::Result;

fn main() ->  Result<()> {
    // read file to string
    let input = include_str!("day5.input");

    // create a vector of queueus of chars
    // let mut crates_queue: Vec<VecDeque<char>> = vec![VecDeque::new(); 3];
    let mut crates_queue: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    input.lines()
        // .take(3)
        .take(9)     // little hack to get the first 9 lines
        .for_each(|line| {
            // split the line into char arrays of size 3
            line.chars().collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(i, chunk)| {
                    if chunk[1] != ' ' {
                        // add the chars to the queue
                        crates_queue[i].push_back(chunk[1]);
                    }
                });
        });

    // enumerate input skip first 5 lines
    input.lines()
        // .skip(5)
        .skip(10)
        .for_each(|line| {
            let words = line.split(" ").collect::<Vec<&str>>();
            let (count, from, to) = (
                words[1].parse::<usize>().unwrap(), 
                words[3].parse::<usize>().unwrap(), 
                words[5].parse::<usize>().unwrap()
            );
            // pop front count times and save in order
            let mut temp = vec!['\0'; count];
            for i in 0..count {
                temp[i] = crates_queue[from - 1].pop_front().unwrap();
            }
            // move crates from one queue to another in same order
            while let Some(c) = temp.pop() {
                crates_queue[to - 1].push_front(c);
            }
        });
    
    // create a string from popped chars
    let mut result = String::new();
    crates_queue.iter_mut()
        .for_each(|q| {
            let popped = q.pop_front();
            result.push(popped.unwrap());
        });
    
    println!("{}", result);

    return Ok(());
}