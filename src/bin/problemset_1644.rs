use std::{
    collections::BTreeSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let a = split.next().unwrap().parse().unwrap();
    let b = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, a, b));
}

fn solve(x: &[i32], a: usize, b: usize) -> i64 {
    let mut prefix_sums = vec![0];
    for &xi in x {
        prefix_sums.push(prefix_sums.last().unwrap() + (xi as i64));
    }

    let mut result = prefix_sums[a];
    let mut subtracted = BTreeSet::from([(prefix_sums[0], 0)]);
    for i in 0..x.len() - a {
        if i >= b - a {
            subtracted.remove(&(prefix_sums[i - (b - a)], i - (b - a)));
        }
        subtracted.insert((prefix_sums[i + 1], i + 1));

        result = result.max(
            (prefix_sums[i + 1] - subtracted.first().unwrap().0)
                + (prefix_sums[i + a + 1] - prefix_sums[i + 1]),
        );
    }

    result
}
