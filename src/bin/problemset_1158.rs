use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let x = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut h = Vec::new();
    for _ in 0..n {
        h.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut s = Vec::new();
    for _ in 0..n {
        s.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&h, &s, x));
}

fn solve(h: &[i32], s: &[i32], x: i32) -> i32 {
    let n = h.len();

    let mut dp = vec![-1; (x as usize) + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in (h[i] as usize..dp.len()).rev() {
            if dp[j - (h[i] as usize)] != -1 {
                dp[j] = dp[j].max(dp[j - (h[i] as usize)] + s[i]);
            }
        }
    }

    dp.iter().max().copied().unwrap()
}
