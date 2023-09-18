use std::{
    io::{stdin, BufRead, BufReader},
    vec,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut x = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b, &x));
}

fn solve(n: usize, a: &[usize], b: &[usize], x: &[i32]) -> i64 {
    let reachables = build_reachables(n, a, b);

    let mut scores = vec![i64::MIN; n];
    scores[0] = 0;
    let mut round = 0;
    loop {
        let mut updated = false;
        for i in 0..a.len() {
            if reachables[b[i] - 1]
                && scores[a[i] - 1] != i64::MIN
                && scores[a[i] - 1] + (x[i] as i64) > scores[b[i] - 1]
            {
                scores[b[i] - 1] = scores[a[i] - 1] + (x[i] as i64);
                updated = true;
            }
        }

        if !updated {
            break;
        }

        if round == n - 1 {
            return -1;
        }
        round += 1;
    }

    scores[n - 1]
}

fn build_reachables(n: usize, a: &[usize], b: &[usize]) -> Vec<bool> {
    let mut adj_vecs: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut reachables = vec![false; n];
    search(&mut reachables, &adj_vecs, n - 1);

    reachables
}

fn search(reachables: &mut [bool], adj_vecs: &[Vec<usize>], node: usize) {
    reachables[node] = true;

    for &adj in &adj_vecs[node] {
        if !reachables[adj] {
            search(reachables, adj_vecs, adj);
        }
    }
}
