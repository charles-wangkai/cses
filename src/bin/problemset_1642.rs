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
    let mut first_two_sum_to_indices = HashMap::new();
    for i in 0..a.len() {
        for j in 0..(i as i32) - 1 {
            first_two_sum_to_indices.insert(
                a[j as usize] + a[i - 1],
                FirstTwoIndices {
                    index1: j as usize,
                    index2: i - 1,
                },
            );
        }

        for j in i + 1..a.len() {
            if let Some(&FirstTwoIndices { index1, index2 }) =
                first_two_sum_to_indices.get(&(x - a[i] - a[j]))
            {
                return format!("{} {} {} {}", index1 + 1, index2 + 1, i + 1, j + 1);
            }
        }
    }

    String::from("IMPOSSIBLE")
}

struct FirstTwoIndices {
    index1: usize,
    index2: usize,
}
