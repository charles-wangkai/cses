use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        x.push(split.next().unwrap().parse().unwrap());
        y.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &y));
}

fn solve(x: &[i32], y: &[i32]) -> String {
    format!(
        "{}\n{}",
        compute_is_containing(x, y)
            .iter()
            .map(|&x| if x { 1 } else { 0 })
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        compute_is_contained(x, y)
            .iter()
            .map(|&x| if x { 1 } else { 0 })
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn compute_is_containing(x: &[i32], y: &[i32]) -> Vec<bool> {
    let mut sorted_indices: Vec<_> = (0..x.len()).collect();
    sorted_indices.sort_by_key(|&i| (y[i], -x[i]));

    let mut result = vec![false; x.len()];
    let mut max_begin = 0;
    for index in sorted_indices {
        if x[index] <= max_begin {
            result[index] = true;
        } else {
            max_begin = x[index];
        }
    }

    result
}

fn compute_is_contained(x: &[i32], y: &[i32]) -> Vec<bool> {
    let mut sorted_indices: Vec<_> = (0..x.len()).collect();
    sorted_indices.sort_by_key(|&i| (x[i], -y[i]));

    let mut result = vec![false; x.len()];
    let mut max_end = 0;
    for index in sorted_indices {
        if y[index] <= max_end {
            result[index] = true;
        } else {
            max_end = y[index];
        }
    }

    result
}
