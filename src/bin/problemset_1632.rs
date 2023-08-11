use std::{
    collections::BTreeSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let k = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b, k));
}

fn solve(a: &[i32], b: &[i32], k: i32) -> i32 {
    let mut sorted_indices: Vec<_> = (0..a.len()).collect();
    sorted_indices.sort_by_key(|&i| b[i]);

    let mut end_times = BTreeSet::new();
    for i in 0..k {
        end_times.insert((0, i));
    }

    let mut result = 0;
    for index in sorted_indices {
        if let Some(&end_time) = end_times.range(..(a[index], i32::MAX)).next_back() {
            end_times.remove(&end_time);
            end_times.insert((b[index], end_time.1));

            result += 1;
        }
    }

    result
}
