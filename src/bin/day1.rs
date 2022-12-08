use anyhow::Result;

fn main() ->  Result<()> {
    // read file to string
    let input = include_str!("day1.input");

    let sums = input.split("\n\n")
        .map(|x| {
            return x.lines()
            .map(|x| x.parse::<usize>().unwrap())
            .sum(); 
        })
        .collect::<Vec<usize>>();

    // create a vector of max 3 elements
    let mut max_3_sums = vec![usize::MIN; 3];

    // find the max 3 sums and store them in the vector
    for sum in sums {
        if sum > max_3_sums[0] {
            max_3_sums[2] = max_3_sums[1];
            max_3_sums[1] = max_3_sums[0];
            max_3_sums[0] = sum;
        } else if sum > max_3_sums[1] {
            max_3_sums[2] = max_3_sums[1];
            max_3_sums[1] = sum;
        } else if sum > max_3_sums[2] {
            max_3_sums[2] = sum;
        }
    }

    println!("max_3_sums: {:?}", max_3_sums);

    println!("{}", max_3_sums.iter().sum::<usize>());

    return Ok(());
}