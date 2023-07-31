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
    let m = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut h = Vec::new();
    for _ in 0..n {
        h.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut t = Vec::new();
    for _ in 0..m {
        t.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&h, &t));
}

fn solve(h: &[i32], t: &[i32]) -> String {
    let mut price_to_count = BTreeMap::new();
    for &hi in h {
        update_map(&mut price_to_count, hi, 1);
    }

    let mut result = Vec::new();
    for ti in t {
        if let Some((&price, _)) = price_to_count.range(..=ti).next_back() {
            update_map(&mut price_to_count, price, -1);
            result.push(price);
        } else {
            result.push(-1);
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn update_map(price_to_count: &mut BTreeMap<i32, i32>, price: i32, delta: i32) {
    price_to_count
        .entry(price)
        .and_modify(|count| *count += delta)
        .or_insert(delta);
    if price_to_count[&price] == 0 {
        price_to_count.remove(&price);
    }
}
