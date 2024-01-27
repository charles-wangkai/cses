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
    let mut binary_indexed_tree = vec![0; (1 << (x.len().ilog2() + 1)) + 1];
    for i in 0..x.len() {
        update(&mut binary_indexed_tree, i + 1, 1);
    }

    let mut result = Vec::new();
    for &pi in p {
        let index = find(x.len(), &binary_indexed_tree, pi);

        update(&mut binary_indexed_tree, index + 1, -1);
        result.push(x[index]);
    }

    result
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn update(binary_indexed_tree: &mut [i32], mut index: usize, delta: i32) {
    while index < binary_indexed_tree.len() {
        binary_indexed_tree[index] += delta;
        index += ((index as i32) & -(index as i32)) as usize;
    }
}

fn find(n: usize, binary_indexed_tree: &[i32], pos: i32) -> usize {
    let mut result = usize::MAX;
    let mut lower = 1;
    let mut upper = n;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if compute_prefix_sum(binary_indexed_tree, middle) >= pos {
            result = middle - 1;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
}

fn compute_prefix_sum(binary_indexed_tree: &[i32], mut index: usize) -> i32 {
    let mut result = 0;
    while index != 0 {
        result += binary_indexed_tree[index];
        index -= ((index as i32) & -(index as i32)) as usize;
    }

    result
}
