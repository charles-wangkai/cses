use std::{
    collections::HashSet,
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
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&mut x, &a, &b));
}

fn solve(x: &mut [usize], a: &[usize], b: &[usize]) -> String {
    let mut sorted_indices: Vec<_> = (0..x.len()).collect();
    sorted_indices.sort_by_key(|&i| x[i]);

    let mut round_num = (0..sorted_indices.len())
        .filter(|&i| i == 0 || sorted_indices[i] < sorted_indices[i - 1])
        .count() as i32;

    let mut result = Vec::new();
    for i in 0..a.len() {
        let value1 = x[a[i] - 1];
        let value2 = x[b[i] - 1];

        x[a[i] - 1] = value2;
        x[b[i] - 1] = value1;

        round_num -= HashSet::from([value1 - 1, value1, value2 - 1, value2])
            .iter()
            .map(|&i| get_inverse_num(&sorted_indices, i))
            .sum::<i32>();

        sorted_indices[value1 - 1] = b[i] - 1;
        sorted_indices[value2 - 1] = a[i] - 1;

        round_num += HashSet::from([value1 - 1, value1, value2 - 1, value2])
            .iter()
            .map(|&i| get_inverse_num(&sorted_indices, i))
            .sum::<i32>();

        result.push(round_num);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_inverse_num(sorted_indices: &[usize], index: usize) -> i32 {
    if index != sorted_indices.len()
        && (index == 0 || sorted_indices[index] < sorted_indices[index - 1])
    {
        1
    } else {
        0
    }
}
