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
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&e, &a, &b));
}

fn solve(e: &[usize], a: &[usize], b: &[usize]) -> String {
    let n = e.len() + 1;

    let mut childrens = vec![Vec::new(); n];
    for i in 0..e.len() {
        childrens[e[i] - 1].push(i + 1);
    }

    let mut depths = vec![0; n];
    search(&mut depths, &childrens, 0, 0);

    let mut ancestors = vec![vec![None; (n.ilog2() as usize) + 1]; n];
    for i in 0..e.len() {
        ancestors[i + 1][0] = Some(e[i] - 1);
    }
    for j in 1..ancestors[0].len() {
        for i in 0..ancestors.len() {
            ancestors[i][j] = ancestors[i][j - 1].and_then(|a| ancestors[a][j - 1])
        }
    }

    (0..a.len())
        .map(|i| find_lca(&depths, &ancestors, a[i] - 1, b[i] - 1) + 1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_lca(depths: &[i32], ancestors: &[Vec<Option<usize>>], mut u: usize, mut v: usize) -> usize {
    if depths[u] < depths[v] {
        return find_lca(depths, ancestors, v, u);
    }

    let depth_diff = depths[u] - depths[v];
    for i in 0..ancestors[0].len() {
        if ((depth_diff >> i) & 1) == 1 {
            u = ancestors[u][i].unwrap();
        }
    }

    if u == v {
        return u;
    }

    for i in (0..ancestors[0].len()).rev() {
        if ancestors[u][i] != ancestors[v][i] {
            u = ancestors[u][i].unwrap();
            v = ancestors[v][i].unwrap();
        }
    }

    ancestors[u][0].unwrap()
}

fn search(depths: &mut [i32], childrens: &[Vec<usize>], node: usize, depth: i32) {
    depths[node] = depth;

    for &child in &childrens[node] {
        search(depths, childrens, child, depth + 1);
    }
}
