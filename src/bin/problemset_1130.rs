use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n - 1 {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b));
}

fn solve(a: &[usize], b: &[usize]) -> i32 {
    let n = a.len() + 1;

    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut dp = vec![[0; 2]; n];
    search(&mut dp, &adj_vecs, usize::MAX, 0);

    let mut result = 0;
    for dp_i in dp {
        for dp_i_j in dp_i {
            result = result.max(dp_i_j);
        }
    }

    result
}

fn search(dp: &mut [[i32; 2]], adj_vecs: &[Vec<usize>], parent: usize, node: usize) {
    for &adj in &adj_vecs[node] {
        if adj != parent {
            search(dp, adj_vecs, node, adj);
            dp[node][0] += dp[adj][0].max(dp[adj][1]);
        }
    }

    for &adj in &adj_vecs[node] {
        if adj != parent {
            dp[node][1] =
                dp[node][1].max(1 + dp[adj][0] + (dp[node][0] - dp[adj][0].max(dp[adj][1])));
        }
    }
}
