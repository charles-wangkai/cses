// https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm

use std::io::{stdin, BufRead, BufReader};

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

fn solve(n: usize, a: &[usize], b: &[usize]) -> String {
    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
    }

    let mut sorted_nodes = Vec::new();
    let mut visited = vec![false; n];
    for i in 0..visited.len() {
        if !visited[i] {
            search1(&adj_vecs, &mut sorted_nodes, &mut visited, i);
        }
    }
    sorted_nodes.reverse();

    let mut reversed_adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        reversed_adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    visited.fill(false);
    search2(&reversed_adj_vecs, &mut visited, sorted_nodes[0]);

    match (1..sorted_nodes.len()).find(|&i| !visited[sorted_nodes[i]]) {
        Some(i) => format!("NO\n{} {}", sorted_nodes[i] + 1, sorted_nodes[0] + 1),
        None => String::from("YES"),
    }
}

fn search2(reversed_adj_vecs: &[Vec<usize>], visited: &mut [bool], node: usize) {
    visited[node] = true;

    for &adj in &reversed_adj_vecs[node] {
        if !visited[adj] {
            search2(reversed_adj_vecs, visited, adj);
        }
    }
}

fn search1(
    adj_vecs: &[Vec<usize>],
    sorted_nodes: &mut Vec<usize>,
    visited: &mut [bool],
    node: usize,
) {
    visited[node] = true;

    for &adj in &adj_vecs[node] {
        if !visited[adj] {
            search1(adj_vecs, sorted_nodes, visited, adj);
        }
    }

    sorted_nodes.push(node);
}
