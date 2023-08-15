use std::io::{stdin, BufRead, BufReader};

const MODULUS: i32 = 1_000_000_007;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, m));
}

fn solve(x: &[i32], m: i32) -> i32 {
    let mut dp = vec![vec![0; (m as usize) + 1]; x.len()];
    if x[0] == 0 {
        for i in 1..=m as usize {
            dp[0][i] = 1;
        }
    } else {
        dp[0][x[0] as usize] = 1;
    }

    for i in 1..x.len() {
        for j in 1..=m as usize {
            if x[i] == j as i32 || x[i] == 0 {
                for d in -1..=1i32 {
                    let prev = (j as i32) + d;
                    if (1..=m).contains(&prev) {
                        dp[i][j] = add_mod(dp[i][j], dp[i - 1][prev as usize]);
                    }
                }
            }
        }
    }

    dp.last().unwrap().iter().copied().reduce(add_mod).unwrap()
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
