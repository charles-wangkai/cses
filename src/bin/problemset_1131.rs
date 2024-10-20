use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n - 1 {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b));
}

fn solve(a: &[usize], b: &[usize]) -> i32 {
    let n = a.len() + 1;

    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    search(&adj_vecs, usize::MAX, search(&adj_vecs, usize::MAX, 0).node).distance
}

struct Outcome {
    distance: i32,
    node: usize,
}

fn search(adj_vecs: &[Vec<usize>], parent: usize, node: usize) -> Outcome {
    let mut result = Outcome { distance: 0, node };
    for &adj in &adj_vecs[node] {
        if adj != parent {
            let outcome = search(adj_vecs, node, adj);
            if outcome.distance + 1 > result.distance {
                result.distance = outcome.distance + 1;
                result.node = outcome.node;
            }
        }
    }

    result
}
