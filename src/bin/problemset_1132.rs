use std::{
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader},
};

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

    let mut subtree_max_distances = vec![0; n];
    search1(&mut subtree_max_distances, &adj_vecs, usize::MAX, 0);

    let mut max_distances = vec![0; n];
    search2(
        &mut max_distances,
        &adj_vecs,
        &subtree_max_distances,
        0,
        usize::MAX,
        0,
    );

    max_distances
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn search1(subtree_max_distances: &mut [i32], adj_vecs: &[Vec<usize>], parent: usize, node: usize) {
    for &adj in &adj_vecs[node] {
        if adj != parent {
            search1(subtree_max_distances, adj_vecs, node, adj);
            subtree_max_distances[node] =
                subtree_max_distances[node].max(subtree_max_distances[adj] + 1);
        }
    }
}

fn search2(
    max_distances: &mut [i32],
    adj_vecs: &[Vec<usize>],
    subtree_max_distances: &[i32],
    parent_distance: i32,
    parent: usize,
    node: usize,
) {
    let mut distance_to_count = BTreeMap::from([(parent_distance, 1)]);
    for &adj in &adj_vecs[node] {
        if adj != parent {
            update_map(&mut distance_to_count, subtree_max_distances[adj] + 1, 1);
        }
    }

    max_distances[node] = *distance_to_count.last_key_value().unwrap().0;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            update_map(&mut distance_to_count, subtree_max_distances[adj] + 1, -1);

            search2(
                max_distances,
                adj_vecs,
                subtree_max_distances,
                distance_to_count.last_key_value().unwrap().0 + 1,
                node,
                adj,
            );

            update_map(&mut distance_to_count, subtree_max_distances[adj] + 1, 1);
        }
    }
}

fn update_map(distance_to_count: &mut BTreeMap<i32, i32>, distance: i32, delta: i32) {
    distance_to_count
        .entry(distance)
        .and_modify(|count| *count += delta)
        .or_insert(delta);

    if distance_to_count[&distance] == 0 {
        distance_to_count.remove(&distance);
    }
}
