use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &p));
}

fn solve(x: &[i32], p: &[i32]) -> String {
    let mut fenwick_tree = FenwickTree::new(x.len());
    for i in 0..x.len() {
        fenwick_tree.add(i + 1, 1);
    }

    let mut result = Vec::new();
    for &pi in p {
        let index = find(x.len(), &fenwick_tree, pi);

        fenwick_tree.add(index + 1, -1);
        result.push(x[index]);
    }

    result
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn find(n: usize, fenwick_tree: &FenwickTree, pos: i32) -> usize {
    let mut result = usize::MAX;
    let mut lower = 1;
    let mut upper = n;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if fenwick_tree.compute_prefix_sum(middle) >= pos {
            result = middle - 1;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
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
