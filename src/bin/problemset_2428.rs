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
    let k = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, k));
}

fn solve(x: &[i32], k: usize) -> i64 {
    let mut result = 0;
    let mut begin_index = 0;
    let mut value_to_count = HashMap::new();
    for i in 0..x.len() {
        update_map(&mut value_to_count, x[i], 1);
        while value_to_count.len() == k + 1 {
            update_map(&mut value_to_count, x[begin_index], -1);
            begin_index += 1;
        }

        result += (i - begin_index + 1) as i64;
    }

    result
}

fn update_map(value_to_count: &mut HashMap<i32, i32>, value: i32, delta: i32) {
    value_to_count
        .entry(value)
        .and_modify(|count| *count += delta)
        .or_insert(delta);
    if value_to_count[&value] == 0 {
        value_to_count.remove(&value);
    }
}
