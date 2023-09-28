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
            search(&adj_vecs, &mut sorted_nodes, &mut visited, i);
        }
    }
    sorted_nodes.reverse();

    let mut distances = vec![-1; n];
    distances[0] = 0;
    let mut prevs = vec![usize::MAX; n];
    for node in sorted_nodes {
        if distances[node] != -1 {
            for &adj in &adj_vecs[node] {
                if distances[node] + 1 > distances[adj] {
                    distances[adj] = distances[node] + 1;
                    prevs[adj] = node;
                }
            }
        }
    }

    if distances[n - 1] == -1 {
        return String::from("IMPOSSIBLE");
    }

    let mut route = Vec::new();
    let mut current = n - 1;
    while current != usize::MAX {
        route.push(current);
        current = prevs[current];
    }
    route.reverse();

    format!(
        "{}\n{}",
        route.len(),
        route
            .iter()
            .map(|x| x + 1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
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
