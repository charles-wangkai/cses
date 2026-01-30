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
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&mut x, &queries));
}

fn solve(x: &mut [i32], queries: &[String]) -> String {
    let mut fenwick_tree = FenwickTree::new(x.len());
    for (i, &x_i) in x.iter().enumerate() {
        fenwick_tree.add(i + 1, x_i);
    }

    let mut result = Vec::new();
    for query in queries {
        let parts: Vec<_> = query.split_whitespace().collect();
        if parts[0] == "1" {
            let k = parts[1].parse().unwrap();
            let u = parts[2].parse().unwrap();

            fenwick_tree.add(k, u - x[k - 1]);
            x[k - 1] = u;
        } else {
            let a: usize = parts[1].parse().unwrap();
            let b = parts[2].parse().unwrap();

            result
                .push(fenwick_tree.compute_prefix_sum(b) - fenwick_tree.compute_prefix_sum(a - 1));
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
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
