// https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm

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
    let mut k = Vec::new();
    for _ in 0..n {
        k.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&k, &a, &b));
}

fn solve(k: &[i32], a: &[usize], b: &[usize]) -> i64 {
    let n = k.len();

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

    let mut component = 0;
    let mut components = vec![usize::MAX; n];
    for node in sorted_nodes {
        if components[node] == usize::MAX {
            search2(&reversed_adj_vecs, &mut components, node, component);
            component += 1;
        }
    }

    let mut sums = vec![0; component];
    for i in 0..k.len() {
        sums[components[i]] += k[i] as i64;
    }

    let mut coin_nums = sums.clone();

    let mut sorted_edges: Vec<_> = (0..a.len()).collect();
    sorted_edges.sort_by_key(|&edge| components[a[edge] - 1]);

    for edge in sorted_edges {
        if components[a[edge] - 1] != components[b[edge] - 1] {
            coin_nums[components[b[edge] - 1]] = coin_nums[components[b[edge] - 1]]
                .max(coin_nums[components[a[edge] - 1]] + sums[components[b[edge] - 1]]);
        }
    }

    coin_nums.iter().max().copied().unwrap()
}

fn search2(
    reversed_adj_vecs: &[Vec<usize>],
    components: &mut [usize],
    node: usize,
    component: usize,
) {
    components[node] = component;

    for &adj in &reversed_adj_vecs[node] {
        if components[adj] == usize::MAX {
            search2(reversed_adj_vecs, components, adj, component);
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
