use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &a, &b));
}

fn solve(x: &[i32], a: &[usize], b: &[usize]) -> String {
    let mut prefix_xor_sums = vec![0; x.len()];
    for i in 0..prefix_xor_sums.len() {
        prefix_xor_sums[i] = (if i == 0 { 0 } else { prefix_xor_sums[i - 1] }) ^ x[i];
    }

    (0..a.len())
        .map(|i| {
            prefix_xor_sums[b[i] - 1]
                ^ (if a[i] == 1 {
                    0
                } else {
                    prefix_xor_sums[a[i] - 2]
                })
        })
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
