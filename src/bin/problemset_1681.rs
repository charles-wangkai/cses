use std::io::{stdin, BufRead, BufReader};

const MOD_INT: ModInt = ModInt {
    modulus: 1_000_000_007,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b));
}

fn solve(n: usize, a: &[usize], b: &[usize]) -> i32 {
    let mut scc = Scc::new(n);
    for i in 0..a.len() {
        scc.add_edge(a[i] - 1, b[i] - 1);
    }

    let sorted_nodes = scc.topological_sort();

    let mut dp = vec![0; n];
    dp[0] = 1;
    for node in sorted_nodes {
        for &adj in &scc.adj_vecs[node] {
            dp[adj] = MOD_INT.add_mod(dp[adj], dp[node]);
        }
    }

    dp[n - 1]
}

struct ModInt {
    modulus: i32,
}

#[allow(dead_code)]
impl ModInt {
    fn modulo(&self, x: i64) -> i32 {
        x.rem_euclid(self.modulus as i64) as i32
    }

    fn mod_inv(&self, x: i32) -> i32 {
        self.pow_mod(x, (self.modulus - 2) as i64)
    }

    fn add_mod(&self, x: i32, y: i32) -> i32 {
        self.modulo((x + y) as i64)
    }

    fn multiply_mod(&self, x: i32, y: i32) -> i32 {
        self.modulo((x as i64) * (y as i64))
    }

    fn divide_mod(&self, x: i32, y: i32) -> i32 {
        self.multiply_mod(x, self.mod_inv(y))
    }

    fn pow_mod(&self, base: i32, exponent: i64) -> i32 {
        if exponent == 0 {
            return 1;
        }

        self.multiply_mod(
            if exponent % 2 == 0 { 1 } else { base },
            self.pow_mod(self.multiply_mod(base, base), exponent / 2),
        )
    }
}

struct Scc {
    adj_vecs: Vec<Vec<usize>>,
    reversed_adj_vecs: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Scc {
    fn new(n: usize) -> Self {
        Self {
            adj_vecs: vec![Vec::new(); n],
            reversed_adj_vecs: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adj_vecs[from].push(to);
        self.reversed_adj_vecs[to].push(from);
    }

    fn topological_sort(&self) -> Vec<usize> {
        let n = self.adj_vecs.len();

        let mut sorted = Vec::new();
        let mut visited = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                self.search1(&mut sorted, &mut visited, i);
            }
        }
        sorted.reverse();

        sorted
    }

    fn search1(&self, sorted: &mut Vec<usize>, visited: &mut [bool], node: usize) {
        visited[node] = true;

        for &adj in &self.adj_vecs[node] {
            if !visited[adj] {
                self.search1(sorted, visited, adj);
            }
        }

        sorted.push(node);
    }

    fn build_components(&self) -> Vec<usize> {
        let n = self.adj_vecs.len();

        let sorted = self.topological_sort();

        let mut components = vec![usize::MAX; n];
        let mut component = 0;
        for node in sorted {
            if components[node] == usize::MAX {
                self.search2(&mut components, component, node);
                component += 1;
            }
        }

        components
    }

    fn search2(&self, components: &mut [usize], component: usize, node: usize) {
        components[node] = component;

        for &adj in &self.reversed_adj_vecs[node] {
            if components[adj] == usize::MAX {
                self.search2(components, component, adj);
            }
        }
    }
}
