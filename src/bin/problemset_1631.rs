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
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&t));
}

fn solve(t: &[i32]) -> i64 {
    let max_time = t.iter().max().copied().unwrap();

    ((2 * max_time) as i64)
        + (t.iter().map(|&x| x as i64).sum::<i64>() - ((2 * max_time) as i64)).max(0)
}
