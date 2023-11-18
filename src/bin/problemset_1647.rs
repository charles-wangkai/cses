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
    let size = (usize::BITS as usize) - (x.len().leading_zeros() as usize);
    let mut mins = vec![vec![0; size]; x.len()];
    for i in 0..x.len() {
        mins[i][0] = x[i];
    }
    for j in 1..size {
        for i in 0..x.len() {
            if i + (1 << j) <= x.len() {
                mins[i][j] = mins[i][j - 1].min(mins[i + (1 << (j - 1))][j - 1]);
            }
        }
    }

    (0..a.len())
        .map(|i| {
            let exp = (usize::BITS as usize) - ((b[i] - a[i] + 1).leading_zeros() as usize) - 1;

            mins[a[i] - 1][exp].min(mins[b[i] - (1 << exp)][exp])
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
