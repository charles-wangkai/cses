use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n - 1 {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }
    let mut u = Vec::new();
    let mut v = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        u.push(split.next().unwrap().parse().unwrap());
        v.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b, &u, &v));
}

fn solve(a: &[usize], b: &[usize], u: &[usize], v: &[usize]) -> String {
    let n = a.len() + 1;

    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut parents = vec![0; n];
    let mut depths = vec![0; n];
    search(&mut parents, &mut depths, &adj_vecs, usize::MAX, 0, 0);

    let mut ancestors = vec![vec![None; (n.ilog2() as usize) + 1]; n];
    for i in 1..parents.len() {
        ancestors[i][0] = Some(parents[i]);
    }
    for j in 1..ancestors[0].len() {
        for i in 0..ancestors.len() {
            ancestors[i][j] = ancestors[i][j - 1].and_then(|a| ancestors[a][j - 1])
        }
    }

    (0..u.len())
        .map(|i| {
            depths[u[i] - 1] + depths[v[i] - 1]
                - depths[find_lca(&depths, &ancestors, u[i] - 1, v[i] - 1)] * 2
        })
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

fn search(
    parents: &mut [usize],
    depths: &mut [i32],
    adj_vecs: &[Vec<usize>],
    parent: usize,
    node: usize,
    depth: i32,
) {
    parents[node] = parent;
    depths[node] = depth;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            search(parents, depths, adj_vecs, node, adj, depth + 1);
        }
    }
}
