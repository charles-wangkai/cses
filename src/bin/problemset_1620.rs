use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let t = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut k = Vec::new();
    for _ in 0..n {
        k.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&k, t));
}

fn solve(k: &[i32], t: i32) -> i64 {
    let mut result = -1;
    let mut lower = 0;
    let mut upper = (k.iter().min().copied().unwrap() as i64) * (t as i64);
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if check(k, t, middle) {
            result = middle;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
}

fn check(k: &[i32], t: i32, time: i64) -> bool {
    k.iter().map(|&ki| time / (ki as i64)).sum::<i64>() >= t as i64
}
