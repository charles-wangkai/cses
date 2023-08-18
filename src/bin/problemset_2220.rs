use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let a = split.next().unwrap().parse().unwrap();
    let b = split.next().unwrap().parse().unwrap();

    println!("{}", solve(a, b));
}

fn solve(a: i64, b: i64) -> i64 {
    compute_valid_num(b + 1) - compute_valid_num(a)
}

fn compute_valid_num(limit: i64) -> i64 {
    let digits: Vec<_> = limit
        .to_string()
        .as_bytes()
        .iter()
        .map(|c| c - b'0')
        .collect();

    let mut tight_dp = vec![0; digits.len()];
    let mut free_dp = vec![vec![0; 10]; digits.len()];
    tight_dp[0] = 1;
    for d in 1..digits[0] {
        free_dp[0][d as usize] = 1;
    }
    for i in 1..digits.len() {
        for d in 0..=digits[i] {
            if d != digits[i - 1] {
                if d == digits[i] {
                    tight_dp[i] += tight_dp[i - 1];
                } else {
                    free_dp[i][d as usize] += tight_dp[i - 1];
                }
            }
        }

        for d in 0..10 {
            for prev_d in 0..10 {
                if prev_d != d {
                    free_dp[i][d] += free_dp[i - 1][prev_d];
                }
            }
        }
    }

    (0..(digits.len() as i32) - if digits[0] == 0 { 1 } else { 0 })
        .map(compute_all_num)
        .sum::<i64>()
        + free_dp.last().unwrap().iter().sum::<i64>()
}

fn compute_all_num(length: i32) -> i64 {
    if length == 0 {
        return 1;
    }

    let mut dp = vec![vec![0; 10]; length as usize];
    for d in 1..10 {
        dp[0][d] = 1;
    }
    for i in 1..dp.len() {
        for d in 0..10 {
            for prev_d in 0..10 {
                if prev_d != d {
                    dp[i][d] += dp[i - 1][prev_d];
                }
            }
        }
    }

    dp.last().unwrap().iter().sum()
}
