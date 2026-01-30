// https://www.cnblogs.com/arizonayyds/articles/offline.html

use std::{
    collections::{hash_map::Entry, HashMap},
    io::{stdin, BufRead, BufReader},
};

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
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &a, &b));
}

fn solve(x: &[i32], a: &[usize], b: &[usize]) -> String {
    let mut sorted_query_indices: Vec<_> = (0..a.len()).collect();
    sorted_query_indices.sort_by_key(|&i| b[i]);

    let mut value_to_last_index = HashMap::new();
    let mut fenwick_tree = FenwickTree::new(x.len());
    let mut end_index = b[sorted_query_indices[0]] - 1;
    for i in (0..=end_index).rev() {
        if let Entry::Vacant(e) = value_to_last_index.entry(x[i]) {
            e.insert(i);
            fenwick_tree.add(i + 1, 1);
        }
    }

    let mut result = vec![0; a.len()];
    for query_index in sorted_query_indices {
        while end_index != b[query_index] - 1 {
            end_index += 1;

            if value_to_last_index.contains_key(&x[end_index]) {
                fenwick_tree.add(value_to_last_index[&x[end_index]] + 1, -1);
            }

            value_to_last_index.insert(x[end_index], end_index);
            fenwick_tree.add(end_index + 1, 1);
        }

        result[query_index] = fenwick_tree.compute_prefix_sum(b[query_index])
            - fenwick_tree.compute_prefix_sum(a[query_index] - 1);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct FenwickTree {
    a: Vec<i32>,
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
            self.a[pos] += delta;
            pos += ((pos as i32) & -(pos as i32)) as usize;
        }
    }

    fn compute_prefix_sum(&self, mut pos: usize) -> i32 {
        let mut result = 0;
        while pos != 0 {
            result += self.a[pos];
            pos -= ((pos as i32) & -(pos as i32)) as usize;
        }

        result
    }
}
