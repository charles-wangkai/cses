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
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&mut x));
}

fn solve(x: &mut [i32]) -> i64 {
    x.sort();

    let mut sum = 0;
    for &mut xi in x {
        if xi as i64 > sum + 1 {
            break;
        }

        sum += xi as i64;
    }

    sum + 1
}
