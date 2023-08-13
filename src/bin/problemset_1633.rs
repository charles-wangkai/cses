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

fn solve(n: i32) -> i32 {
    let mut dp = vec![0; (n as usize) + 1];
    dp[0] = 1;
    for i in 1..dp.len() {
        for j in 1..=6.min(i) {
            dp[i] = add_mod(dp[i], dp[i - j]);
        }
    }

    dp[n as usize]
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
