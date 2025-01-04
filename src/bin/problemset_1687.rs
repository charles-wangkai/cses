use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut e = Vec::new();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    for _ in 0..n - 1 {
        e.push(split.next().unwrap().parse().unwrap());
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

    println!("{}", solve(&e, &x, &k));
}

fn solve(e: &[usize], x: &[usize], k: &[i32]) -> String {
    let n = e.len() + 1;

    let mut ancestors = vec![vec![None; (n.ilog2() as usize) + 1]; n];
    for i in 0..e.len() {
        ancestors[i + 1][0] = Some(e[i] - 1);
    }
    for j in 1..ancestors[0].len() {
        for i in 0..ancestors.len() {
            ancestors[i][j] = ancestors[i][j - 1].and_then(|a| ancestors[a][j - 1]);
        }
    }

    (0..x.len())
        .map(|i| {
            let mut node = x[i] - 1;
            for j in 0..ancestors[0].len() {
                if ((k[i] >> j) & 1) == 1 {
                    match ancestors[node][j] {
                        Some(a) => {
                            node = a;
                        }
                        None => return -1,
                    }
                }
            }

            (node as i32) + 1
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
