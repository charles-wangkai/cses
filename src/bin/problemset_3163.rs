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
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    let mut d = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        c.push(split.next().unwrap().parse().unwrap());
        d.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &a, &b, &c, &d));
}

fn solve(x: &[i32], a: &[usize], b: &[usize], c: &[i32], d: &[i32]) -> String {
    let n = x.len();
    let q = a.len();

    let mut sorted_indices = (0..x.len()).collect::<Vec<_>>();
    sorted_indices.sort_by_key(|&i| x[i]);

    let mut events = (0..q)
        .flat_map(|query_index| {
            vec![
                Event {
                    value: d[query_index],
                    pos: b[query_index],
                    sign: 1,
                    query_index,
                },
                Event {
                    value: d[query_index],
                    pos: a[query_index] - 1,
                    sign: -1,
                    query_index,
                },
                Event {
                    value: c[query_index] - 1,
                    pos: b[query_index],
                    sign: -1,
                    query_index,
                },
                Event {
                    value: c[query_index] - 1,
                    pos: a[query_index] - 1,
                    sign: 1,
                    query_index,
                },
            ]
        })
        .collect::<Vec<_>>();
    events.sort_by_key(|event| event.value);

    let mut result = vec![0; q];
    let mut fenwick_tree = FenwickTree::new(n);
    let mut seq = 0;
    for event in &events {
        while seq != sorted_indices.len() && x[sorted_indices[seq]] <= event.value {
            fenwick_tree.add(sorted_indices[seq] + 1, 1);
            seq += 1;
        }

        result[event.query_index] += event.sign * fenwick_tree.compute_prefix_sum(event.pos);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct Event {
    value: i32,
    pos: usize,
    sign: i32,
    query_index: usize,
}

struct FenwickTree {
    a: Vec<i32>,
}

#[allow(dead_code)]
impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            a: vec![0; size + 1],
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
