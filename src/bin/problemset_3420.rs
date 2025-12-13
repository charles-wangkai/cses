use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
};

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

fn solve(x: &[i32]) -> i64 {
    let mut result = 0;
    let mut end_index = -1;
    let mut seen = HashSet::new();
    for begin_index in 0..x.len() {
        while end_index != (x.len() as i32) - 1 && !seen.contains(&x[(end_index + 1) as usize]) {
            end_index += 1;
            seen.insert(x[end_index as usize]);
        }

        result += (end_index - (begin_index as i32) + 1) as i64;

        seen.remove(&x[begin_index]);
    }

    result
}
