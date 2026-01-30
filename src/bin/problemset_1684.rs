use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut symbol1s = Vec::new();
    let mut topping1s = Vec::new();
    let mut symbol2s = Vec::new();
    let mut topping2s = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        symbol1s.push(split.next().unwrap().parse().unwrap());
        topping1s.push(split.next().unwrap().parse().unwrap());
        symbol2s.push(split.next().unwrap().parse().unwrap());
        topping2s.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&symbol1s, &topping1s, &symbol2s, &topping2s, m));
}

fn solve(
    symbol1s: &[char],
    topping1s: &[usize],
    symbol2s: &[char],
    topping2s: &[usize],
    m: usize,
) -> String {
    let mut two_sat = TwoSat::new(m);
    for i in 0..symbol1s.len() {
        two_sat.add_clause(
            topping1s[i] - 1,
            symbol1s[i] == '+',
            topping2s[i] - 1,
            symbol2s[i] == '+',
        );
    }

    match two_sat.find_assignment() {
        Some(assignment) => assignment
            .iter()
            .map(|&x| if x { "+" } else { "-" })
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        None => "IMPOSSIBLE".to_string(),
    }
}

struct TwoSat {
    n: usize,
    scc: Scc,
}

#[allow(dead_code)]
impl TwoSat {
    fn new(n: usize) -> Self {
        Self {
            n,
            scc: Scc::new(2 * n),
        }
    }

    fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.scc.add_edge(
            2 * i + (if f { 0 } else { 1 }),
            2 * j + (if g { 1 } else { 0 }),
        );
        self.scc.add_edge(
            2 * j + (if g { 0 } else { 1 }),
            2 * i + (if f { 1 } else { 0 }),
        );
    }

    fn find_assignment(&self) -> Option<Vec<bool>> {
        let components = self.scc.build_components();

        let mut assignment = vec![false; self.n];
        for i in 0..assignment.len() {
            if components[2 * i] == components[2 * i + 1] {
                return None;
            }

            assignment[i] = components[2 * i] < components[2 * i + 1];
        }

        Some(assignment)
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
