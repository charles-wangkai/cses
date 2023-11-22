use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        c.push(split.next().unwrap().parse().unwrap());
    }
    let mut qa = Vec::new();
    let mut qb = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        qa.push(split.next().unwrap().parse().unwrap());
        qb.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b, &c, &qa, &qb));
}

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32], qa: &[usize], qb: &[usize]) -> String {
    let mut distances = vec![vec![i64::MAX; n]; n];
    for (i, distances_i) in distances.iter_mut().enumerate() {
        distances_i[i] = 0;
    }
    for i in 0..a.len() {
        distances[a[i] - 1][b[i] - 1] = distances[a[i] - 1][b[i] - 1].min(c[i] as i64);
        distances[b[i] - 1][a[i] - 1] = distances[b[i] - 1][a[i] - 1].min(c[i] as i64);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances[i][k] != i64::MAX && distances[k][j] != i64::MAX {
                    distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                }
            }
        }
    }

    (0..qa.len())
        .map(|i| {
            if distances[qa[i] - 1][qb[i] - 1] == i64::MAX {
                -1
            } else {
                distances[qa[i] - 1][qb[i] - 1]
            }
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
