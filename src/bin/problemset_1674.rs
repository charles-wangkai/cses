use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut bosses = Vec::new();
    for _ in 0..n - 1 {
        bosses.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&bosses));
}

fn solve(bosses: &[usize]) -> String {
    let n = bosses.len() + 1;

    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..bosses.len() {
        adj_vecs[bosses[i] - 1].push(i + 1);
    }

    let mut subordinate_nums = vec![0; n];
    search(&mut subordinate_nums, &adj_vecs, 0);

    subordinate_nums
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn search(subordinate_nums: &mut [i32], adj_vecs: &[Vec<usize>], node: usize) -> i32 {
    for &adj in &adj_vecs[node] {
        subordinate_nums[node] += search(subordinate_nums, adj_vecs, adj);
    }

    subordinate_nums[node] + 1
}
