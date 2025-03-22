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

    let mut binary_indexed_tree = vec![0; (1 << ((2 * n).ilog2() + 1)) + 1];
    for i in 0..v.len() {
        update(&mut binary_indexed_tree, out_times[i], v[i]);
    }

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let s: usize = split.next().unwrap().parse().unwrap();
            let x = split.next().unwrap().parse().unwrap();

            update(&mut binary_indexed_tree, out_times[s - 1], x - v[s - 1]);
            v[s - 1] = x;
        } else {
            let s: usize = split.next().unwrap().parse().unwrap();

            result.push(
                compute_prefix_sum(&binary_indexed_tree, out_times[s - 1])
                    - compute_prefix_sum(&binary_indexed_tree, in_times[s - 1]),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn compute_prefix_sum(binary_indexed_tree: &[i64], mut index: usize) -> i64 {
    let mut result = 0;
    while index != 0 {
        result += binary_indexed_tree[index];
        index -= ((index as i32) & -(index as i32)) as usize;
    }

    result
}

fn update(binary_indexed_tree: &mut [i64], mut index: usize, delta: i32) {
    while index < binary_indexed_tree.len() {
        binary_indexed_tree[index] += delta as i64;
        index += ((index as i32) & -(index as i32)) as usize;
    }
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
