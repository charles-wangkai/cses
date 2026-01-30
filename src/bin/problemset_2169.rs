use std::{
    collections::{BTreeSet, HashMap},
    io::{stdin, BufRead, BufReader},
    iter::FromIterator,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        x.push(split.next().unwrap().parse().unwrap());
        y.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &y));
}

fn solve(x: &[i32], y: &[i32]) -> String {
    let sorted_values: Vec<_> = x
        .iter()
        .chain(y.iter())
        .copied()
        .collect::<BTreeSet<_>>()
        .iter()
        .copied()
        .collect();
    let value_to_compressed: HashMap<_, _> =
        HashMap::from_iter((0..sorted_values.len()).map(|i| (sorted_values[i], i + 1)));

    let x: Vec<_> = x.iter().map(|xi| value_to_compressed[xi]).collect();
    let y: Vec<_> = y.iter().map(|yi| value_to_compressed[yi]).collect();

    format!(
        "{}\n{}",
        compute_containing_nums(&x, &y)
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        compute_contained_nums(&x, &y)
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn compute_containing_nums(x: &[usize], y: &[usize]) -> Vec<i32> {
    let mut sorted_indices: Vec<_> = (0..x.len()).collect();
    sorted_indices.sort_by_key(|&i| (y[i], -(x[i] as i32)));

    let mut result = vec![0; x.len()];
    let mut fenwick_tree = FenwickTree::new(x.iter().chain(y.iter()).max().copied().unwrap());
    for index in sorted_indices {
        result[index] = fenwick_tree.compute_prefix_sum(fenwick_tree.a.len() - 1)
            - fenwick_tree.compute_prefix_sum(x[index] - 1);
        fenwick_tree.add(x[index], 1);
    }

    result
}

fn compute_contained_nums(x: &[usize], y: &[usize]) -> Vec<i32> {
    let mut sorted_indices: Vec<_> = (0..x.len()).collect();
    sorted_indices.sort_by_key(|&i| (x[i], -(y[i] as i32)));

    let mut result = vec![0; x.len()];
    let mut fenwick_tree = FenwickTree::new(x.iter().chain(y.iter()).max().copied().unwrap());
    for index in sorted_indices {
        result[index] = fenwick_tree.compute_prefix_sum(fenwick_tree.a.len() - 1)
            - fenwick_tree.compute_prefix_sum(y[index] - 1);
        fenwick_tree.add(y[index], 1);
    }

    result
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
