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
    let q = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&p, &queries));
}

fn solve(p: &[i32], queries: &[String]) -> String {
    let value_to_compressed = build_value_to_compressed(p, queries);

    let mut fenwick_tree = FenwickTree::new(value_to_compressed.len());

    let mut salaries = Vec::new();
    for pi in p {
        salaries.push(value_to_compressed[pi]);
        fenwick_tree.add(value_to_compressed[pi] as usize, 1);
    }

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: char = split.next().unwrap().parse().unwrap();
        if kind == '!' {
            let k: usize = split.next().unwrap().parse().unwrap();
            let x = split.next().unwrap().parse().unwrap();

            fenwick_tree.add(salaries[k - 1] as usize, -1);
            salaries[k - 1] = value_to_compressed[&x];
            fenwick_tree.add(value_to_compressed[&x] as usize, 1);
        } else {
            let a: i32 = split.next().unwrap().parse().unwrap();
            let b = split.next().unwrap().parse().unwrap();

            result.push(
                fenwick_tree.compute_prefix_sum(value_to_compressed[&b] as usize)
                    - fenwick_tree.compute_prefix_sum(value_to_compressed[&(a - 1)] as usize),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_value_to_compressed(p: &[i32], queries: &[String]) -> HashMap<i32, i32> {
    let mut sorted_values = BTreeSet::new();
    for &pi in p {
        sorted_values.insert(pi);
    }
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: char = split.next().unwrap().parse().unwrap();
        if kind == '!' {
            split.next().unwrap();
            let x = split.next().unwrap().parse().unwrap();

            sorted_values.insert(x);
        } else {
            let a: i32 = split.next().unwrap().parse().unwrap();
            let b = split.next().unwrap().parse().unwrap();

            sorted_values.insert(a - 1);
            sorted_values.insert(b);
        }
    }

    HashMap::from_iter(
        sorted_values
            .iter()
            .enumerate()
            .map(|(i, &value)| (value, (i as i32) + 1)),
    )
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
