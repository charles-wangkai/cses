use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let t = split.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let a = split.next().unwrap().parse().unwrap();
        let b = split.next().unwrap().parse().unwrap();

        println!("{}", if solve(a, b) { "YES" } else { "NO" });
    }
}

fn solve(a: i32, b: i32) -> bool {
    (a + b) % 3 == 0 && ((a + b) / 3) <= a.min(b)
}
