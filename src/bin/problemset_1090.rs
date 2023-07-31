use std::{
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let x = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&p, x));
}

fn solve(p: &[i32], x: i32) -> i32 {
    let mut weight_to_count = BTreeMap::new();
    for &pi in p {
        update_map(&mut weight_to_count, pi, 1);
    }

    let mut result = 0;
    while let Some((&max_weight, _)) = weight_to_count.last_key_value() {
        update_map(&mut weight_to_count, max_weight, -1);

        if let Some((&other_weight, _)) = weight_to_count.range(..=x - max_weight).next_back() {
            update_map(&mut weight_to_count, other_weight, -1);
        }

        result += 1;
    }

    result
}

fn update_map(weight_to_count: &mut BTreeMap<i32, i32>, weight: i32, delta: i32) {
    weight_to_count
        .entry(weight)
        .and_modify(|count| *count += delta)
        .or_insert(delta);
    if weight_to_count[&weight] == 0 {
        weight_to_count.remove(&weight);
    }
}
