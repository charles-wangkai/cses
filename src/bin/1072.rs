// https://oeis.org/A172132

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
    (1..=n)
        .map(|i| ((i as i64) - 1) * ((i as i64) + 4) * ((i.pow(2) as i64) - 3 * (i as i64) + 4) / 2)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
