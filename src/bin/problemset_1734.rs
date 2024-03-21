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
    let mut binary_indexed_tree = vec![0; (1 << (x.len().ilog2() + 1)) + 1];
    let mut end_index = b[sorted_query_indices[0]] - 1;
    for i in (0..=end_index).rev() {
        if let Entry::Vacant(e) = value_to_last_index.entry(x[i]) {
            e.insert(i);
            update(&mut binary_indexed_tree, i + 1, 1);
        }
    }

    let mut result = vec![0; a.len()];
    for query_index in sorted_query_indices {
        while end_index != b[query_index] - 1 {
            end_index += 1;

            if value_to_last_index.contains_key(&x[end_index]) {
                update(
                    &mut binary_indexed_tree,
                    value_to_last_index[&x[end_index]] + 1,
                    -1,
                );
            }

            value_to_last_index.insert(x[end_index], end_index);
            update(&mut binary_indexed_tree, end_index + 1, 1);
        }

        result[query_index] = compute_prefix_sum(&binary_indexed_tree, b[query_index])
            - compute_prefix_sum(&binary_indexed_tree, a[query_index] - 1);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn update(binary_indexed_tree: &mut [i32], mut index: usize, delta: i32) {
    while index < binary_indexed_tree.len() {
        binary_indexed_tree[index] += delta;
        index += ((index as i32) & -(index as i32)) as usize;
    }
}

fn compute_prefix_sum(binary_indexed_tree: &[i32], mut index: usize) -> i32 {
    let mut result = 0;
    while index != 0 {
        result += binary_indexed_tree[index];
        index -= ((index as i32) & -(index as i32)) as usize;
    }

    result
}
