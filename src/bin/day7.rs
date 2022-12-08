use anyhow::Result;

fn main() ->  Result<()> {
    let input = include_str!("day7.input");

    // create a queue to store current path
    let mut current_path = vec![];
    // create map from current_path to size
    let mut map = std::collections::HashMap::new();
    map.insert(String::from("/"), 0);
    // input skip 1 line and split into groups that starts with a $
    for line in input.lines() {
        let mut line = line.split_whitespace();
        // if line starts with a $, then it is a command
        let first = line.next().unwrap();
        if first.starts_with("$") {
            let command = line.next().unwrap();
            // if command is cd, then change current path
            if command == "cd" {
                let path = line.next().unwrap();
                if path == ".." {
                    current_path.pop();
                } else {
                    // TODO: save full path
                    current_path.push(path);
                }
                // println!("{:?}", current_path);
            }
        } else if is_number(&first) {
                // if line does not start with dir, then it is a file
                let size = first.parse::<i32>().unwrap();

                // for each current path, add size to the map
                for i in 0..current_path.len() {
                    // can be avoided by saving full paths (problem with borrow checker)
                    let mut path = String::new();
                    for j in 0..i {
                        path += current_path[j];
                        path += "/";
                    }
                    path += current_path[i];

                    println!("{}", path);
                    
                    *map.entry(path).or_insert(0) += size;
                }
        } else {
            // skip line
            continue;
        }
    }

    // count sum of all values in map less than 100000
    // let sum = map.values().filter(|x| **x < 100000).sum::<i32>();

    // fin largest value in map
    let total_used = map.values().max().unwrap();
    // max between total_used - 30000000 and 0
    let must_free =total_used - 40000000;

    println!("must_free: {}", must_free);

    // find smallest value in map that is greater than must_free
    let mut smallest: i32 = i32::MAX;
    for value in map.values() {
        if *value > must_free && value < &smallest {
            smallest = *value;
        }
    }

    println!("{}", smallest);

    return Ok(());
}

// check if str is a number
fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}