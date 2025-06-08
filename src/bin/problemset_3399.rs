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
        let n = split.next().unwrap().parse().unwrap();
        let a = split.next().unwrap().parse().unwrap();
        let b = split.next().unwrap().parse().unwrap();

        println!("{}", solve(n, a, b));
    }
}

fn solve(n: i32, a: i32, b: i32) -> String {
    if a + b > n || ((a == 0) ^ (b == 0)) {
        return "NO".to_string();
    }

    let cards1: Vec<_> = (1..=n).collect();
    let cards2: Vec<_> = (0..n)
        .map(|i| {
            if i < a + b {
                (i + a) % (a + b) + 1
            } else {
                i + 1
            }
        })
        .collect();

    format!(
        "YES\n{}\n{}",
        cards1
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        cards2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    )
}
