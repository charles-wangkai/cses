use std::io::{stdin, BufRead, BufReader};

const MODULUS: i32 = 1_000_000_007;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b));
}

fn solve(n: usize, a: &[usize], b: &[usize]) -> i32 {
    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
    }

    let mut dp = vec![vec![0; n]; 1 << n];
    dp[1][0] = 1;
    for mask in 1..1 << n {
        for node in 0..n {
            if dp[mask][node] != 0 {
                for &adj in &adj_vecs[node] {
                    if ((mask >> adj) & 1) == 0 {
                        let next_mask = mask | (1 << adj);
                        dp[next_mask][adj] = add_mod(dp[next_mask][adj], dp[mask][node]);
                    }
                }
            }
        }
    }

    dp[(1 << n) - 1][n - 1]
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
