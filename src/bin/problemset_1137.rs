// https://github.com/Jonathan-Uy/CSES-Solutions/blob/main/Tree%20Algorithms/Subtree%20Queries.cpp

use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n - 1 {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&mut v, &a, &b, &queries));
}

fn solve(v: &mut [i32], a: &[usize], b: &[usize], queries: &[String]) -> String {
    let n = v.len();

    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut in_times = vec![0; n];
    let mut out_times = vec![0; n];
    search(
        &mut in_times,
        &mut out_times,
        &adj_vecs,
        usize::MAX,
        0,
        &mut 0,
    );

    let mut fenwick_tree = FenwickTree::new(2 * n);
    for i in 0..v.len() {
        fenwick_tree.add(out_times[i], v[i]);
    }

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let s: usize = split.next().unwrap().parse().unwrap();
            let x = split.next().unwrap().parse().unwrap();

            fenwick_tree.add(out_times[s - 1], x - v[s - 1]);
            v[s - 1] = x;
        } else {
            let s: usize = split.next().unwrap().parse().unwrap();

            result.push(
                fenwick_tree.compute_prefix_sum(out_times[s - 1])
                    - fenwick_tree.compute_prefix_sum(in_times[s - 1]),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn search(
    in_times: &mut [usize],
    out_times: &mut [usize],
    adj_vecs: &[Vec<usize>],
    parent: usize,
    node: usize,
    time: &mut usize,
) {
    in_times[node] = *time;
    *time += 1;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            search(in_times, out_times, adj_vecs, node, adj, time);
        }
    }

    out_times[node] = *time;
    *time += 1;
}

struct FenwickTree {
    a: Vec<i64>,
}

#[allow(dead_code)]
impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            a: vec![0; (1 << (size.ilog2() + 1)) + 1],
        }
    }

    fn add(&mut self, mut pos: usize, delta: i32) {
        while pos < self.a.len() {
            self.a[pos] += delta as i64;
            pos += ((pos as i32) & -(pos as i32)) as usize;
        }
    }

    fn compute_prefix_sum(&self, mut pos: usize) -> i64 {
        let mut result = 0;
        while pos != 0 {
            result += self.a[pos];
            pos -= ((pos as i32) & -(pos as i32)) as usize;
        }

        result
    }
}
