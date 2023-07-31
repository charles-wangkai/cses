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

    println!("{}", solve(&x));
}

fn solve(x: &[i32]) -> usize {
    let mut sorted_indices: Vec<_> = (0..x.len()).collect();
    sorted_indices.sort_by_key(|&i| x[i]);

    (0..sorted_indices.len())
        .filter(|&i| i == 0 || sorted_indices[i] < sorted_indices[i - 1])
        .count()
}
