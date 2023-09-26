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
    let k = split.next().unwrap().parse().unwrap();
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

    println!("{}", solve(n, &a, &b, &c, k));
}

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32], k: usize) -> String {
    let mut edge_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        edge_vecs[a[i] - 1].push(i);
    }

    let mut result = Vec::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    let mut counts = vec![0; n];
    loop {
        let (Reverse(price), node) = heap.pop().unwrap();
        if node == n - 1 {
            result.push(price);

            if result.len() == k {
                return result
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }

        counts[node] += 1;
        if counts[node] <= k {
            for &edge in &edge_vecs[node] {
                heap.push((Reverse(price + (c[edge] as i64)), b[edge] - 1));
            }
        }
    }
}
