use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let s1: String = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let s2: String = split.next().unwrap().parse().unwrap();

    println!("{}", solve(&s1, &s2));
}

fn solve(s1: &str, s2: &str) -> i32 {
    let mut dp = vec![vec![i32::MAX; s2.len() + 1]; s1.len() + 1];
    dp[0][0] = 0;
    for i in 0..=s1.len() {
        for j in 0..=s2.len() {
            if i != 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }
            if j != 0 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }
            if i != 0 && j != 0 {
                dp[i][j] = dp[i][j].min(
                    dp[i - 1][j - 1]
                        + if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                            0
                        } else {
                            1
                        },
                );
            }
        }
    }

    dp[s1.len()][s2.len()]
}
