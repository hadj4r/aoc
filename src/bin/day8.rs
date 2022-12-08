use anyhow::Result;

fn main() ->  Result<()> {
    let input = include_str!("day8.input");
    let mut matrix: Vec<Vec<i8>> = vec![];
    let mut pairs: Vec<(usize, usize)> = vec![];
    let mut sum = 0;

    // init matrix
    for line in input.lines() {
        let mut row: Vec<i8> = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i8);
        }
        matrix.push(row);
    }

    // horizontal
    for (i, row) in matrix.iter().enumerate() {
        let mut start = 0;
        let mut last_left = 0;
        let mut max_left: i8 = -1;

        // 30373
        while start < row.len() {
            let left = row[start];
            if max_left < left {
                max_left = left;
                last_left = start;
                pairs.push((i, start));
            }
            
            start += 1;
        }

        let mut end = row.len() - 1;
        let mut max_right: i8 = -1;

        while end > last_left {
            let right = row[end];
            if max_right < right {
                max_right = right;
                pairs.push((i, end));
            }

            end -= 1;
        }
    }

    // vertical
    for i in 0..matrix[0].len() {
        let mut start = 0;
        let mut last_top = 0;
        let mut max_top: i8 = -1;

        while start < matrix.len() {
            let top = matrix[start][i];
            if max_top < top {
                max_top = top;
                last_top = start;
                pairs.push((start, i));
            }
            
            start += 1;
        }

        let mut end = matrix.len() - 1;
        let mut max_bottom: i8 = -1;

        while end > last_top {
            let bottom = matrix[end][i];
            if max_bottom < bottom {
                max_bottom = bottom;
                pairs.push((end, i));
            }

            end -= 1;
        }
    }

    // count unique pairs
    pairs.sort();
    pairs.dedup();

    println!("sum: {}", pairs.len());

    return Ok(());
}

