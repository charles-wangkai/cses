use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut b = Vec::new();
    for _ in 0..m {
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b));
}

fn solve(a: &[i32], b: &[i32]) -> String {
    let n = a.len();
    let m = b.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut common = Vec::new();
    let mut length1 = n;
    let mut length2 = m;
    while dp[length1][length2] != 0 {
        if a[length1 - 1] == b[length2 - 1] {
            common.push(a[length1 - 1]);

            length1 -= 1;
            length2 -= 1;
        } else if dp[length1 - 1][length2] >= dp[length1][length2 - 1] {
            length1 -= 1;
        } else {
            length2 -= 1;
        }
    }
    common.reverse();

    format!(
        "{}\n{}",
        common.len(),
        common
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
