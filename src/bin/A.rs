#![allow(non_snake_case)]

use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let A = split.next().unwrap().parse().unwrap();
    let B = split.next().unwrap().parse().unwrap();

    println!("{}", solve(A, B));
}

fn solve(A: i32, B: i32) -> i32 {
    A + B
}
