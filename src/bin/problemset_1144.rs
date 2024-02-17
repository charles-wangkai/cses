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

    let mut binary_indexed_tree = vec![0; (1 << (value_to_compressed.len().ilog2() + 1)) + 1];

    let mut salaries = Vec::new();
    for pi in p {
        salaries.push(value_to_compressed[pi]);
        update(
            &mut binary_indexed_tree,
            value_to_compressed[pi] as usize,
            1,
        );
    }

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: char = split.next().unwrap().parse().unwrap();
        if kind == '!' {
            let k: usize = split.next().unwrap().parse().unwrap();
            let x = split.next().unwrap().parse().unwrap();

            update(&mut binary_indexed_tree, salaries[k - 1] as usize, -1);
            salaries[k - 1] = value_to_compressed[&x];
            update(
                &mut binary_indexed_tree,
                value_to_compressed[&x] as usize,
                1,
            );
        } else {
            let a: i32 = split.next().unwrap().parse().unwrap();
            let b = split.next().unwrap().parse().unwrap();

            result.push(
                compute_prefix_sum(&binary_indexed_tree, value_to_compressed[&b] as usize)
                    - compute_prefix_sum(
                        &binary_indexed_tree,
                        value_to_compressed[&(a - 1)] as usize,
                    ),
            );
        }
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
