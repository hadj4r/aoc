use anyhow::Result;

fn main() ->  Result<()> {
    let input = include_str!("day8.input");
    let mut matrix: Vec<Vec<i8>> = vec![];
    let mut max_score = 0;

    // init matrix
    for line in input.lines() {
        let mut row: Vec<i8> = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i8);
        }
        matrix.push(row);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        for j in 0..cols {
            // compare matrix[i][j] with all above
            let mut up = 0;
            for k in (0..i).rev() {
                up += 1;
                if matrix[i][j] <= matrix[k][j] {
                    break;
                }
            }

            // compare matrix[i][j] with all below
            let mut down = 0;
            for k in i+1..rows {
                down += 1;
                if matrix[i][j] <= matrix[k][j] {
                    break;
                }
            }

            // compare matrix[i][j] with all left
            let mut left = 0;
            for k in (0..j).rev() {
                left += 1;
                if matrix[i][j] <= matrix[i][k] {
                    break;
                }
            }

            // compare matrix[i][j] with all right
            let mut right = 0;
            for k in j+1..cols {
                right += 1;
                if matrix[i][j] <= matrix[i][k] {
                    break;
                }
            }

            max_score = std::cmp::max(max_score, up * down * left * right);
        }
    }
  
    println!("max score: {}", max_score);

    return Ok(());
}

