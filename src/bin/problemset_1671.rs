use std::{
    cmp::Reverse,
    collections::BinaryHeap,
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
    let mut c = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        c.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b, &c));
}

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32]) -> String {
    let mut edge_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        edge_vecs[a[i] - 1].push(i);
    }

    let mut distances = vec![i64::MAX; n];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), 0));
    while let Some((Reverse(distance), node)) = pq.pop() {
        if distances[node] == i64::MAX {
            distances[node] = distance;
            for &edge in &edge_vecs[node] {
                if distances[b[edge] - 1] == i64::MAX {
                    pq.push((Reverse(distances[node] + (c[edge] as i64)), b[edge] - 1));
                }
            }
        }
    }

    distances
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
