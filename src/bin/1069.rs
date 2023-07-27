use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let s: String = split.next().unwrap().parse().unwrap();

    println!("{}", solve(&s));
}

fn solve(s: &str) -> i32 {
    let mut result = 0;
    let letters: Vec<_> = s.chars().collect();
    let mut length = 0;
    for i in 0..letters.len() {
        if i != 0 && letters[i] == letters[i - 1] {
            length += 1;
        } else {
            length = 1;
        }

        result = result.max(length);
    }

    result
}
