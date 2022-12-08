use std::vec;

use anyhow::Result;

fn main() ->  Result<()> {
    let input = include_str!("day6.input");
    let mut last_4_chars = vec!['\0'; 14];
    let mut last_index = 0;

    // TODO: Should try to to not recreate a hashset every time
    for (i, c) in input.chars().enumerate() {
        last_4_chars[last_index] = c;
        let set: std::collections::HashSet<_> = last_4_chars.iter().collect();
        if set.len() == 14 && i > 13 {
            println!("{}", i + 1);
            break;
        }
        last_index = (last_index + 1) % 14;
    }

    return Ok(());
}