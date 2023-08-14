use std::io::{stdin, BufRead, BufReader};

const MODULUS: i32 = 1_000_000_007;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let x = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut c = Vec::new();
    for _ in 0..n {
        c.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&c, x));
}

fn solve(c: &[i32], x: i32) -> i32 {
    let mut dp = vec![0; (x as usize) + 1];
    dp[0] = 1;
    for &ci in c {
        for i in ci as usize..dp.len() {
            dp[i] = add_mod(dp[i], dp[i - (ci as usize)]);
        }
    }

    dp.last().copied().unwrap()
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
