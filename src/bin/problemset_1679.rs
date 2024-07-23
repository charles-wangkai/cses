use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
    iter::FromIterator,
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
            search(&adj_vecs, &mut sorted_nodes, &mut visited, i);
        }
    }
    sorted_nodes.reverse();

    let node_to_index: HashMap<_, _> =
        HashMap::from_iter((0..sorted_nodes.len()).map(|i| (sorted_nodes[i], i)));
    if (0..a.len()).any(|i| node_to_index[&(a[i] - 1)] >= node_to_index[&(b[i] - 1)]) {
        return String::from("IMPOSSIBLE");
    }

    sorted_nodes
        .iter()
        .map(|x| x + 1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn search(
    adj_vecs: &[Vec<usize>],
    sorted_nodes: &mut Vec<usize>,
    visited: &mut [bool],
    node: usize,
) {
    visited[node] = true;

    for &adj in &adj_vecs[node] {
        if !visited[adj] {
            search(adj_vecs, sorted_nodes, visited, adj);
        }
    }

    sorted_nodes.push(node);
}
