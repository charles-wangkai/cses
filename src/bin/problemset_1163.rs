use std::{
    collections::{BTreeMap, BTreeSet},
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let x = split.next().unwrap().parse().unwrap();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(x, &p));
}

fn solve(x: i32, p: &[i32]) -> String {
    let mut traffic_lights = BTreeSet::from([0, x]);
    let mut length_to_count = BTreeMap::from([(x, 1)]);

    let mut result = Vec::new();
    for &pi in p {
        let &left = traffic_lights.range(..pi).next_back().unwrap();
        let &right = traffic_lights.range(pi..).next().unwrap();

        traffic_lights.insert(pi);

        update_map(&mut length_to_count, right - left, -1);
        update_map(&mut length_to_count, pi - left, 1);
        update_map(&mut length_to_count, right - pi, 1);

        result.push(*length_to_count.last_key_value().unwrap().0);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn update_map(length_to_count: &mut BTreeMap<i32, i32>, length: i32, delta: i32) {
    length_to_count
        .entry(length)
        .and_modify(|count| *count += delta)
        .or_insert(delta);
    if length_to_count[&length] == 0 {
        length_to_count.remove(&length);
    }
}
