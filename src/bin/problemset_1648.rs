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
    let mut binary_indexed_tree = vec![0; (1 << (usize::BITS - x.len().leading_zeros())) + 1];
    for (i, &x_i) in x.iter().enumerate() {
        update(&mut binary_indexed_tree, i + 1, x_i);
    }

    let mut result = Vec::new();
    for query in queries {
        let parts: Vec<_> = query.split_whitespace().collect();
        if parts[0] == "1" {
            let k = parts[1].parse().unwrap();
            let u = parts[2].parse().unwrap();

            update(&mut binary_indexed_tree, k, u - x[k - 1]);
            x[k - 1] = u;
        } else {
            let a: usize = parts[1].parse().unwrap();
            let b = parts[2].parse().unwrap();

            result.push(
                compute_prefix_sum(&binary_indexed_tree, b)
                    - compute_prefix_sum(&binary_indexed_tree, a - 1),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn compute_prefix_sum(binary_indexed_tree: &[i64], mut index: usize) -> i64 {
    let mut result = 0;
    while index != 0 {
        result += binary_indexed_tree[index];
        index -= ((index as i32) & -(index as i32)) as usize;
    }

    result
}

fn update(binary_indexed_tree: &mut [i64], mut index: usize, delta: i32) {
    while index < binary_indexed_tree.len() {
        binary_indexed_tree[index] += delta as i64;
        index += ((index as i32) & -(index as i32)) as usize;
    }
}
