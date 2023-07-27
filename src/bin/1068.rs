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
    let mut result = Vec::new();
    let mut current = n as i64;
    loop {
        result.push(current);
        if current == 1 {
            break;
        }

        current = if current % 2 == 0 {
            current / 2
        } else {
            current * 3 + 1
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
