use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
    iter::FromIterator,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b));
}

fn solve(n: i32, a: &[i32], b: &[i32]) -> String {
    let mut parents = vec![-1; n as usize];
    for i in 0..a.len() {
        let a_root = find_root(&mut parents, a[i] - 1);
        let b_root = find_root(&mut parents, b[i] - 1);
        if a_root != b_root {
            parents[b_root as usize] = a_root;
        }
    }

    let roots = Vec::from_iter(
        (0..n)
            .map(|i| find_root(&mut parents, i))
            .collect::<HashSet<_>>(),
    );

    format!(
        "{}\n{}",
        roots.len() - 1,
        (0..roots.len() - 1)
            .map(|i| format!("{} {}", roots[i] + 1, roots[i + 1] + 1))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn find_root(parents: &mut [i32], node: i32) -> i32 {
    if parents[node as usize] == -1 {
        return node;
    }

    parents[node as usize] = find_root(parents, parents[node as usize]);

    parents[node as usize]
}
