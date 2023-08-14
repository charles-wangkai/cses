use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: i32) -> i32 {
    let mut dp = vec![i32::MAX; (n as usize) + 1];
    dp[0] = 0;
    for i in 1..dp.len() {
        for &c in i.to_string().as_bytes().iter().filter(|&&c| c != b'0') {
            dp[i] = dp[i].min(dp[i - ((c - b'0') as usize)] + 1);
        }
    }

    dp.last().copied().unwrap()
}
