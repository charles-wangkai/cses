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
    let mut w = Vec::new();
    for _ in 0..n {
        w.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&w, x));
}

fn solve(w: &[i32], x: i32) -> i32 {
    let mut result = 0;
    let mut dp = vec![-1; 1 << w.len()];
    dp[0] = 0;
    loop {
        result += 1;

        for rest in dp.iter_mut() {
            if *rest != -1 {
                *rest = x;
            }
        }

        for (i, wi) in w.iter().enumerate() {
            for mask in (0..dp.len()).rev() {
                if dp[mask] != -1 {
                    let next_mask = mask | (1 << i);
                    dp[next_mask] = dp[next_mask].max(dp[mask] - wi);
                }
            }
        }

        if dp.last().copied().unwrap() != -1 {
            return result;
        }
    }
}
