use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: i32) -> String {
    let mut result = vec![String::new()];
    for _ in 0..n {
        result = result
            .iter()
            .map(|x| format!("0{}", x))
            .chain(result.iter().rev().map(|x| format!("1{}", x)))
            .collect();
    }

    result.join("\n")
}
