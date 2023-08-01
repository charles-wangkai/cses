use std::{
    collections::VecDeque,
    io::{stdin, BufRead, BufReader},
    iter::FromIterator,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: i32) -> String {
    let mut result = Vec::new();
    let mut rest = VecDeque::from_iter(1..=n);
    while let Some(head) = rest.pop_front() {
        rest.push_back(head);
        result.push(rest.pop_front().unwrap());
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
