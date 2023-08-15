use std::{
    io::{stdin, BufRead, BufReader},
    sync::Mutex,
};

const LIMIT: usize = 1_000_000;
const MODULUS: i32 = 1_000_000_007;

static DP: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn main() {
    precompute();

    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let t = split.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let n = split.next().unwrap().parse().unwrap();

        println!("{}", solve(n));
    }
}

fn precompute() {
    let mut dp = DP.lock().unwrap();

    *dp = vec![0; LIMIT + 1];
    dp[0] = 1;

    let mut sum = 1;
    let mut progression_sum = 1;
    for dpi in dp.iter_mut().skip(1) {
        *dpi = add_mod(sum, progression_sum);

        sum = add_mod(sum, *dpi);
        progression_sum = add_mod(multiply_mod(progression_sum, 3), *dpi);
    }
}

fn solve(n: usize) -> i32 {
    DP.lock().unwrap()[n]
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}

fn multiply_mod(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64)).rem_euclid(MODULUS as i64) as i32
}
