// https://cp-algorithms.com/graph/2SAT.html

use std::{
    cmp::Ordering,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut symbol1s = Vec::new();
    let mut topping1s = Vec::new();
    let mut symbol2s = Vec::new();
    let mut topping2s = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        symbol1s.push(split.next().unwrap().parse().unwrap());
        topping1s.push(split.next().unwrap().parse().unwrap());
        symbol2s.push(split.next().unwrap().parse().unwrap());
        topping2s.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&symbol1s, &topping1s, &symbol2s, &topping2s, m));
}

fn solve(
    symbol1s: &[char],
    topping1s: &[usize],
    symbol2s: &[char],
    topping2s: &[usize],
    m: usize,
) -> String {
    let mut adj_vecs = vec![Vec::new(); m * 2];
    for i in 0..symbol1s.len() {
        let index1 = (topping1s[i] - 1) * 2 + (if symbol1s[i] == '+' { 0 } else { 1 });
        let index2 = (topping2s[i] - 1) * 2 + (if symbol2s[i] == '+' { 0 } else { 1 });

        adj_vecs[index1 ^ 1].push(index2);
        adj_vecs[index2 ^ 1].push(index1);
    }

    let mut sorted_nodes = Vec::new();
    let mut visited = vec![false; m * 2];
    for i in 0..visited.len() {
        if !visited[i] {
            search1(&adj_vecs, &mut sorted_nodes, &mut visited, i);
        }
    }
    sorted_nodes.reverse();

    let mut reversed_adj_vecs = vec![Vec::new(); m * 2];
    for i in 0..symbol1s.len() {
        let index1 = (topping1s[i] - 1) * 2 + (if symbol1s[i] == '+' { 0 } else { 1 });
        let index2 = (topping2s[i] - 1) * 2 + (if symbol2s[i] == '+' { 0 } else { 1 });

        reversed_adj_vecs[index2].push(index1 ^ 1);
        reversed_adj_vecs[index1].push(index2 ^ 1);
    }

    let mut k = 0;
    let mut components = vec![0; m * 2];
    for node in sorted_nodes {
        if components[node] == 0 {
            k += 1;
            search2(&reversed_adj_vecs, &mut components, node, k);
        }
    }

    let mut result = vec!['\0'; m];
    for i in 0..m {
        result[i] = match components[i * 2].cmp(&components[i * 2 + 1]) {
            Ordering::Less => '-',
            Ordering::Equal => return String::from("IMPOSSIBLE"),
            Ordering::Greater => '+',
        };
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn search2(reversed_adj_vecs: &[Vec<usize>], components: &mut [usize], node: usize, k: usize) {
    components[node] = k;

    for &adj in &reversed_adj_vecs[node] {
        if components[adj] == 0 {
            search2(reversed_adj_vecs, components, adj, k);
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
