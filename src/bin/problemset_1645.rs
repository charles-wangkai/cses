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

fn solve(x: &[i32]) -> String {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    for i in 0..x.len() {
        while let Some(&top) = stack.last() {
            if x[top] < x[i] {
                break;
            }

            stack.pop();
        }

        result.push(match stack.last() {
            Some(top) => top + 1,
            None => 0,
        });

        stack.push(i);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
