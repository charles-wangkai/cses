use std::{
    cmp::Reverse,
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut h = Vec::new();
    for _ in 0..n {
        h.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&h));
}

fn solve(h: &[i32]) -> i32 {
    let mut value_to_indices = BTreeMap::new();
    for i in 0..h.len() {
        value_to_indices
            .entry(Reverse(h[i]))
            .or_insert(Vec::new())
            .push(i);
    }

    let mut index_to_depth = BTreeMap::new();
    for value in value_to_indices.keys() {
        let indices = value_to_indices.get(value).unwrap();
        let mut depths = vec![0; indices.len()];
        for i in 0..indices.len() {
            let left_depth = match index_to_depth.range(..indices[i]).next_back() {
                Some(left) => *left.1,
                None => 0,
            };
            let right_depth = match index_to_depth.range(indices[i] + 1..).next() {
                Some(right) => *right.1,
                None => 0,
            };

            depths[i] = left_depth.max(right_depth) + 1;
        }

        for i in 0..indices.len() {
            index_to_depth.insert(indices[i], depths[i]);
        }
    }

    index_to_depth.values().copied().max().unwrap()
}
