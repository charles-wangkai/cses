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
    let x = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, x));
}

fn solve(a: &[i32], x: i32) -> String {
    let mut value_to_index = HashMap::new();
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if let Some(index) = value_to_index.get(&(x - a[i] - a[j])) {
                return format!("{} {} {}", index + 1, i + 1, j + 1);
            }
        }

        value_to_index.insert(a[i], i);
    }

    String::from("IMPOSSIBLE")
}
