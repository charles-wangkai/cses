use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut a = Vec::new();
    for _ in 0..n - 1 {
        a.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a));
}

fn solve(a: &Vec<i32>) -> i32 {
    let n = (a.len() as i32) + 1;

    (1..=n)
        .chain(a.iter().copied())
        .reduce(|acc, e| acc ^ e)
        .unwrap()
}
