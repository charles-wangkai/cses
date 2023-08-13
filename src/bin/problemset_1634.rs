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
    let mut c = Vec::new();
    for _ in 0..n {
        c.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&c, x));
}

fn solve(c: &[i32], x: i32) -> i32 {
    let mut dp = vec![i32::MAX; (x as usize) + 1];
    dp[0] = 0;
    for i in 1..dp.len() {
        for &ci in c {
            if ci <= i as i32 && dp[i - (ci as usize)] != i32::MAX {
                dp[i] = dp[i].min(dp[i - (ci as usize)] + 1);
            }
        }
    }

    if dp[x as usize] == i32::MAX {
        -1
    } else {
        dp[x as usize]
    }
}
