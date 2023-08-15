use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let a = split.next().unwrap().parse().unwrap();
    let b = split.next().unwrap().parse().unwrap();

    println!("{}", solve(a, b));
}

fn solve(a: usize, b: usize) -> i32 {
    let mut dp = vec![vec![i32::MAX; b + 1]; a + 1];
    for i in 1..=a {
        for j in 1..=b {
            if j == i {
                dp[i][j] = 0;
            } else {
                for k in 1..i {
                    dp[i][j] = dp[i][j].min(dp[k][j] + dp[i - k][j] + 1);
                }
                for k in 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[i][j - k] + 1);
                }
            }
        }
    }

    dp[a][b]
}
