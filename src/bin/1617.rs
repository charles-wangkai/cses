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
    (0..n).fold(1, |acc, _| multiply_mod(acc, 2))
}

fn multiply_mod(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64) % (MODULUS as i64)) as i32
}
