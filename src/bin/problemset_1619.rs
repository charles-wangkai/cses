use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b));
}

fn solve(a: &[i32], b: &[i32]) -> usize {
    let mut sorted_indices: Vec<_> = (0..a.len()).collect();
    sorted_indices.sort_by_key(|&i| a[i]);

    let mut result = 0;
    let mut leaving_times = BinaryHeap::new();
    for index in sorted_indices {
        while let Some(&Reverse(leaving_time)) = leaving_times.peek() {
            if leaving_time > a[index] {
                break;
            }

            leaving_times.pop();
        }

        leaving_times.push(Reverse(b[index]));

        result = result.max(leaving_times.len());
    }

    result
}
