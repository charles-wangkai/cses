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
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&mut p));
}

fn solve(p: &mut [i32]) -> i64 {
    p.sort();

    let mut result = 0;
    let mut left_index = 0;
    let mut right_index = p.len() - 1;
    while left_index < right_index {
        result += (p[right_index] - p[left_index]) as i64;

        left_index += 1;
        right_index -= 1;
    }

    result
}
