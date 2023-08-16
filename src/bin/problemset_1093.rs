use std::io::{stdin, BufRead, BufReader};

const MODULUS: i32 = 1_000_000_007;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: usize) -> i32 {
    let sum = n * (n + 1) / 2;
    if sum % 2 == 1 {
        return 0;
    }

    let mut dp = vec![0; sum / 2 + 1];
    dp[1] = 1;
    for i in 2..=n {
        for j in (i..dp.len()).rev() {
            dp[j] = add_mod(dp[j], dp[j - i]);
        }
    }

    dp.last().copied().unwrap()
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
