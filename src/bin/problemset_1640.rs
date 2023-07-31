use std::{
    cmp::Ordering,
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
    let mut sorted_indices: Vec<_> = (0..a.len()).collect();
    sorted_indices.sort_by_key(|&i| a[i]);

    let mut left = 0;
    let mut right = sorted_indices.len() - 1;
    while left < right {
        match (a[sorted_indices[left]] + a[sorted_indices[right]]).cmp(&x) {
            Ordering::Less => left += 1,
            Ordering::Equal => {
                return format!("{} {}", sorted_indices[left] + 1, sorted_indices[right] + 1)
            }
            Ordering::Greater => right -= 1,
        }
    }

    String::from("IMPOSSIBLE")
}
