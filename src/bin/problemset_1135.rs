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
    let tree = Tree::new(
        &a.iter().map(|ai| ai - 1).collect::<Vec<_>>(),
        &b.iter().map(|bi| bi - 1).collect::<Vec<_>>(),
        0,
    );

    (0..u.len())
        .map(|i| {
            tree.depths[u[i] - 1] + tree.depths[v[i] - 1]
                - tree.depths[tree.find_lca(u[i] - 1, v[i] - 1)] * 2
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
struct Tree {
    n: usize,
    u: Vec<usize>,
    v: Vec<usize>,
    root: usize,
    edge_vecs: Vec<Vec<usize>>,
    depths: Vec<i32>,
    ancestors: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Tree {
    fn new(u: &[usize], v: &[usize], root: usize) -> Self {
        let n = u.len() + 1;

        let mut edge_vecs = vec![Vec::new(); n];
        for i in 0..u.len() {
            edge_vecs[u[i]].push(i);
            edge_vecs[v[i]].push(i);
        }

        let mut depths = vec![0; n];
        let mut ancestors = vec![vec![usize::MAX; (n.ilog2() as usize) + 1]; n];
        Self::init(
            &mut depths,
            &mut ancestors,
            u,
            v,
            &edge_vecs,
            0,
            usize::MAX,
            root,
        );

        Self {
            n,
            u: u.to_vec(),
            v: v.to_vec(),
            root,
            edge_vecs,
            depths,
            ancestors,
        }
    }

    fn init(
        depths: &mut [i32],
        ancestors: &mut [Vec<usize>],
        u: &[usize],
        v: &[usize],
        edge_vecs: &[Vec<usize>],
        depth: i32,
        parent: usize,
        node: usize,
    ) {
        depths[node] = depth;

        ancestors[node][0] = parent;
        for i in 1..ancestors[node].len() {
            if ancestors[node][i - 1] != usize::MAX {
                ancestors[node][i] = ancestors[ancestors[node][i - 1]][i - 1];
            }
        }

        for &edge in &edge_vecs[node] {
            let adj = if node == u[edge] { v[edge] } else { u[edge] };

            if adj != parent {
                Self::init(depths, ancestors, u, v, edge_vecs, depth + 1, node, adj);
            }
        }
    }

    fn find_lca(&self, mut node1: usize, mut node2: usize) -> usize {
        if self.depths[node1] < self.depths[node2] {
            return self.find_lca(node2, node1);
        }

        for i in (0..self.ancestors[node1].len()).rev() {
            if self.ancestors[node1][i] != usize::MAX
                && self.depths[self.ancestors[node1][i]] >= self.depths[node2]
            {
                node1 = self.ancestors[node1][i];
            }
        }

        if node1 == node2 {
            return node1;
        }

        for i in (0..self.ancestors[0].len()).rev() {
            if self.ancestors[node1][i] != self.ancestors[node2][i] {
                node1 = self.ancestors[node1][i];
                node2 = self.ancestors[node2][i];
            }
        }

        self.ancestors[node1][0]
    }
}
