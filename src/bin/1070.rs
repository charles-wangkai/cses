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
    if n == 2 || n == 3 {
        return String::from("NO SOLUTION");
    }

    let mut result = vec![0; n as usize];
    let mut current = 1;
    for i in (1..n).step_by(2) {
        result[i as usize] = current;
        current += 1;
    }
    for i in (0..n).step_by(2) {
        result[i as usize] = current;
        current += 1;
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
