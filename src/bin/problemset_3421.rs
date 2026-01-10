use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

const MODULUS: i32 = 1_000_000_007;

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

fn solve(x: &[i32]) -> i32 {
    let mut value_to_count = HashMap::new();
    for value in x {
        value_to_count
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    value_to_count
        .values()
        .fold(1, |acc, count| multiply_mod(acc, count + 1))
        - 1
}

fn multiply_mod(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64)).rem_euclid(MODULUS as i64) as i32
}
