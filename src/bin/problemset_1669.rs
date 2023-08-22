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
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut parents = vec![usize::MAX; n];
    for i in 0..parents.len() {
        if parents[i] == usize::MAX {
            if let Some(Element { node1, node2 }) = search(&adj_vecs, &mut parents, i, i) {
                let mut path1 = build_path(&parents, node1);
                let mut path2 = build_path(&parents, node2);

                while path1.len() >= 2
                    && path2.len() >= 2
                    && path1[path1.len() - 2] == path2[path2.len() - 2]
                {
                    path1.pop();
                    path2.pop();
                }

                return format!(
                    "{}\n{}",
                    path1.len() + path2.len(),
                    path1
                        .iter()
                        .rev()
                        .chain(path2.iter())
                        .map(|node| node + 1)
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
    }

    String::from("IMPOSSIBLE")
}

fn build_path(parents: &[usize], mut node: usize) -> Vec<usize> {
    let mut result = vec![node];
    while parents[node] != node {
        result.push(parents[node]);
        node = parents[node];
    }

    result
}

fn search(
    adj_vecs: &[Vec<usize>],
    parents: &mut [usize],
    parent: usize,
    node: usize,
) -> Option<Element> {
    parents[node] = parent;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            if parents[adj] != usize::MAX {
                return Some(Element {
                    node1: node,
                    node2: adj,
                });
            }

            let sub_result = search(adj_vecs, parents, node, adj);
            if sub_result.is_some() {
                return sub_result;
            }
        }
    }

    None
}

struct Element {
    node1: usize,
    node2: usize,
}
