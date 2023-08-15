use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x));
}

fn solve(x: &[i32]) -> i64 {
    let n = x.len();

    let mut prefix_sums = vec![0];
    for &xi in x {
        prefix_sums.push(prefix_sums.last().copied().unwrap() + (xi as i64));
    }

    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = x[i] as i64;
    }
    for length in 2..=n {
        for begin_index in 0..=n - length {
            let end_index = begin_index + length - 1;

            dp[begin_index][end_index] = ((x[begin_index] as i64)
                + ((prefix_sums[end_index + 1] - prefix_sums[begin_index + 1])
                    - dp[begin_index + 1][end_index]))
                .max(
                    (x[end_index] as i64)
                        + ((prefix_sums[end_index] - prefix_sums[begin_index])
                            - dp[begin_index][end_index - 1]),
                );
        }
    }

    dp[0][n - 1]
}
