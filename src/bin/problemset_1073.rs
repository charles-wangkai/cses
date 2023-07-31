use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut k = Vec::new();
    for _ in 0..n {
        k.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&k));
}

fn solve(k: &[i32]) -> usize {
    let mut tops = Vec::new();
    for &ki in k {
        let index = find_first_larger_index(&tops, ki);
        if index == -1 {
            tops.push(ki);
        } else {
            tops[index as usize] = ki;
        }
    }

    tops.len()
}

fn find_first_larger_index(tops: &[i32], cube: i32) -> i32 {
    let mut result = -1;
    let mut lower = 0;
    let mut upper = (tops.len() as i32) - 1;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if tops[middle as usize] > cube {
            result = middle;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
}
