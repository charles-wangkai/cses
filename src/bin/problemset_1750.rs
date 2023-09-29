use std::io::{stdin, BufRead, BufReader};

const BIT_NUM: usize = 30;

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
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(split.next().unwrap().parse().unwrap());
    }
    let mut x = Vec::new();
    let mut k = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        x.push(split.next().unwrap().parse().unwrap());
        k.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&t, &x, &k));
}

fn solve(t: &[usize], x: &[usize], k: &[usize]) -> String {
    let mut destinations = vec![[usize::MAX; BIT_NUM]; t.len()];
    for i in 0..destinations.len() {
        destinations[i][0] = t[i] - 1;
    }
    for b in 1..BIT_NUM {
        for i in 0..destinations.len() {
            destinations[i][b] = destinations[destinations[i][b - 1]][b - 1];
        }
    }

    (0..x.len())
        .map(|i| {
            let mut current = x[i] - 1;
            for j in 0..BIT_NUM {
                if ((k[i] >> j) & 1) == 1 {
                    current = destinations[current][j];
                }
            }

            current + 1
        })
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
