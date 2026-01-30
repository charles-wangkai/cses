use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut k = Vec::new();
    for _ in 0..n {
        k.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&k, &a, &b));
}

fn solve(k: &[i32], a: &[usize], b: &[usize]) -> i64 {
    let n = k.len();

    let mut scc = Scc::new(n);
    for i in 0..a.len() {
        scc.add_edge(a[i] - 1, b[i] - 1);
    }

    let components = scc.build_components();
    let component_num = components.iter().max().unwrap() + 1;

    let mut sums = vec![0; component_num];
    for i in 0..k.len() {
        sums[components[i]] += k[i] as i64;
    }

    let mut coin_nums = sums.clone();

    let mut sorted_edges: Vec<_> = (0..a.len()).collect();
    sorted_edges.sort_by_key(|&edge| components[a[edge] - 1]);

    for edge in sorted_edges {
        if components[a[edge] - 1] != components[b[edge] - 1] {
            coin_nums[components[b[edge] - 1]] = coin_nums[components[b[edge] - 1]]
                .max(coin_nums[components[a[edge] - 1]] + sums[components[b[edge] - 1]]);
        }
    }

    coin_nums.iter().max().copied().unwrap()
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
