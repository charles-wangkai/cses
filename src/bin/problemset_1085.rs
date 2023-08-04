use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let k = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, k));
}

fn solve(x: &[i32], k: i32) -> i64 {
    let mut result = -1;
    let mut lower = x.iter().map(|&xi| xi as i64).max().unwrap();
    let mut upper = x.iter().map(|&xi| xi as i64).sum::<i64>();
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if check(x, k, middle) {
            result = middle;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
}

fn check(x: &[i32], k: i32, sum_limit: i64) -> bool {
    let mut division_count = 0;
    let mut rest = 0;
    for &xi in x {
        if xi as i64 > rest {
            division_count += 1;
            rest = sum_limit;
        }

        rest -= xi as i64;
    }

    division_count <= k
}
