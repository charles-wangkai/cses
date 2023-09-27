use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
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

    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            if let Some(route) = search(
                &adj_vecs,
                &mut visited,
                &mut Vec::new(),
                &mut HashSet::new(),
                i,
            ) {
                return format!(
                    "{}\n{}",
                    route.len(),
                    route
                        .iter()
                        .map(|x| x + 1)
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
    }

    String::from("IMPOSSIBLE")
}

fn search(
    adj_vecs: &[Vec<usize>],
    visited: &mut [bool],
    path: &mut Vec<usize>,
    seen: &mut HashSet<usize>,
    node: usize,
) -> Option<Vec<usize>> {
    visited[node] = true;

    path.push(node);
    seen.insert(node);

    for &adj in &adj_vecs[node] {
        if !visited[adj] {
            let sub_result = search(adj_vecs, visited, path, seen, adj);
            if sub_result.is_some() {
                return sub_result;
            }
        } else if seen.contains(&adj) {
            let mut index = 0;
            while path[index] != adj {
                index += 1;
            }

            let mut route = Vec::from(&path[index..]);
            route.push(adj);

            return Some(route);
        }
    }

    seen.remove(&node);
    path.pop();

    None
}
