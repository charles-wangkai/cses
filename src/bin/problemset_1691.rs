// https://cp-algorithms.com/graph/euler_path.html

use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
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

fn solve(n: usize, a: &[usize], b: &[usize]) -> String {
    let mut adj_sets = vec![HashSet::new(); n];
    for i in 0..a.len() {
        adj_sets[a[i] - 1].insert(b[i] - 1);
        adj_sets[b[i] - 1].insert(a[i] - 1);
    }

    if adj_sets.iter().any(|adj_set| adj_set.len() % 2 == 1) {
        return String::from("IMPOSSIBLE");
    }

    let mut route = Vec::new();
    find_euler_path(&mut route, &mut adj_sets, 0);

    if route.len() == a.len() + 1 {
        route
            .iter()
            .map(|x| x + 1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        String::from("IMPOSSIBLE")
    }
}

fn find_euler_path(route: &mut Vec<usize>, adj_sets: &mut [HashSet<usize>], node: usize) {
    while let Some(&adj) = adj_sets[node].iter().next() {
        adj_sets[node].remove(&adj);
        adj_sets[adj].remove(&node);

        find_euler_path(route, adj_sets, adj);
    }

    route.push(node);
}
