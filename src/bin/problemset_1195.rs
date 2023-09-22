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

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32]) -> i64 {
    let mut edge_vecs: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..a.len() {
        edge_vecs[a[i] - 1].push(i);
    }

    let mut no_discount_distances = vec![i64::MAX; n];
    let mut has_discount_distances = vec![i64::MAX; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, false));
    while let Some((Reverse(distance), node, has_discount)) = heap.pop() {
        if has_discount {
            if has_discount_distances[node] == i64::MAX {
                has_discount_distances[node] = distance;

                for &edge in &edge_vecs[node] {
                    if has_discount_distances[b[edge] - 1] == i64::MAX {
                        heap.push((Reverse(distance + (c[edge] as i64)), b[edge] - 1, true));
                    }
                }
            }
        } else if no_discount_distances[node] == i64::MAX {
            no_discount_distances[node] = distance;

            for &edge in &edge_vecs[node] {
                if no_discount_distances[b[edge] - 1] == i64::MAX {
                    heap.push((Reverse(distance + (c[edge] as i64)), b[edge] - 1, false));
                }
                if has_discount_distances[b[edge] - 1] == i64::MAX {
                    heap.push((
                        Reverse(distance + ((c[edge] / 2) as i64)),
                        b[edge] - 1,
                        true,
                    ));
                }
            }
        }
    }

    has_discount_distances[n - 1]
}
