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
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&mut t, &queries));
}

fn solve(t: &mut [i32], queries: &[String]) -> String {
    let mut fenwick_tree = FenwickTree::new(t.len());
    for (i, &t_i) in t.iter().enumerate() {
        fenwick_tree.add(i + 1, t_i);
    }

    let mut last_indices = vec![0];
    let mut adj_vecs = vec![Vec::new(); queries.len() + 1];
    for (i, query_i) in queries.iter().enumerate() {
        let parts: Vec<_> = query_i.split_whitespace().collect();
        let k: usize = parts[1].parse().unwrap();

        adj_vecs[last_indices[k - 1]].push(i + 1);

        if parts[0] == "3" {
            last_indices.push(i + 1);
        } else {
            last_indices[k - 1] = i + 1;
        }
    }

    let mut outcomes = vec![None; adj_vecs.len()];
    search(&mut outcomes, t, queries, &adj_vecs, &mut fenwick_tree, 0);

    outcomes
        .iter()
        .filter(|outcome| outcome.is_some())
        .map(|outcome| outcome.unwrap().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn search(
    outcomes: &mut [Option<i64>],
    t: &mut [i32],
    queries: &[String],
    adj_vecs: &[Vec<usize>],
    fenwick_tree: &mut FenwickTree,
    node: usize,
) {
    let mut old_value = 0;

    if node != 0 {
        let parts: Vec<_> = queries[node - 1].split_whitespace().collect();
        if parts[0] == "1" {
            let a = parts[2].parse().unwrap();
            let x = parts[3].parse().unwrap();

            fenwick_tree.add(a, x - t[a - 1]);
            old_value = t[a - 1];
            t[a - 1] = x;
        } else if parts[0] == "2" {
            let a: usize = parts[2].parse().unwrap();
            let b = parts[3].parse().unwrap();

            outcomes[node] =
                Some(fenwick_tree.compute_prefix_sum(b) - fenwick_tree.compute_prefix_sum(a - 1));
        }
    }

    for &adj in &adj_vecs[node] {
        search(outcomes, t, queries, adj_vecs, fenwick_tree, adj);
    }

    if node != 0 {
        let parts: Vec<_> = queries[node - 1].split_whitespace().collect();
        if parts[0] == "1" {
            let a = parts[2].parse().unwrap();

            fenwick_tree.add(a, old_value - t[a - 1]);
            t[a - 1] = old_value;
        }
    }
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
