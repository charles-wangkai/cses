use std::{
    collections::HashMap,
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
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a));
}

fn solve(a: &[i32]) -> i64 {
    let mut result = 0;
    let mut prefix_sum_mod_to_count = HashMap::from([(0, 1)]);
    let mut prefix_sum = 0;
    for &ai in a {
        prefix_sum += ai as i64;
        let prefix_sum_mod = prefix_sum.rem_euclid(a.len() as i64);

        result += prefix_sum_mod_to_count
            .get(&prefix_sum_mod)
            .copied()
            .unwrap_or(0);

        prefix_sum_mod_to_count
            .entry(prefix_sum_mod)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    result
}
