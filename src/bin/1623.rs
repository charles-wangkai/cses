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

    println!("{}", solve(p));
}

fn solve(p: Vec<i32>) -> i64 {
    (0..1 << p.len())
        .map(|mask| {
            (0..p.len())
                .map(|i| (if ((mask >> i) & 1) == 1 { 1 } else { -1 }) * p[i])
                .map(|x| x as i64)
                .sum::<i64>()
                .abs()
        })
        .min()
        .unwrap()
}
