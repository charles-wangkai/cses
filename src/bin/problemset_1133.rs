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

fn solve(a: &[usize], b: &[usize]) -> String {
    let n = a.len() + 1;

    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut subtree_sizes = vec![0; n];
    search1(&mut subtree_sizes, &adj_vecs, usize::MAX, 0);

    let mut distance_sums = vec![0; n];
    distance_sums[0] = search2(&adj_vecs, usize::MAX, 0, 0);

    search3(&mut distance_sums, &adj_vecs, &subtree_sizes, usize::MAX, 0);

    distance_sums
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn search3(
    distance_sums: &mut [i64],
    adj_vecs: &[Vec<usize>],
    subtree_sizes: &[i32],
    parent: usize,
    node: usize,
) {
    for &adj in &adj_vecs[node] {
        if adj != parent {
            distance_sums[adj] = distance_sums[node] - (subtree_sizes[adj] as i64)
                + (((distance_sums.len() as i32) - subtree_sizes[adj]) as i64);

            search3(distance_sums, adj_vecs, subtree_sizes, node, adj);
        }
    }
}

fn search2(adj_vecs: &[Vec<usize>], parent: usize, node: usize, depth: i32) -> i64 {
    let mut result = depth as i64;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            result += search2(adj_vecs, node, adj, depth + 1);
        }
    }

    result
}

fn search1(subtree_sizes: &mut [i32], adj_vecs: &[Vec<usize>], parent: usize, node: usize) {
    subtree_sizes[node] = 1;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            search1(subtree_sizes, adj_vecs, node, adj);
            subtree_sizes[node] += subtree_sizes[adj];
        }
    }
}
