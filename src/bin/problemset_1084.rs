use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let k = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut b = Vec::new();
    for _ in 0..m {
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&mut a, &mut b, k));
}

fn solve(a: &mut [i32], b: &mut [i32], k: i32) -> i32 {
    a.sort();
    b.sort();

    let mut result = 0;
    let mut b_index = 0;
    for &mut ai in a {
        while b_index != b.len() && b[b_index] < ai - k {
            b_index += 1;
        }

        if b_index != b.len() && b[b_index] <= ai + k {
            result += 1;
            b_index += 1;
        }
    }

    result
}
