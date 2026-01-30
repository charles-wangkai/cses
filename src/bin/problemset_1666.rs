use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
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

fn solve(n: i32, a: &[i32], b: &[i32]) -> String {
    let mut dsu = Dsu::new(n as usize);
    for i in 0..a.len() {
        dsu.union((a[i] as usize) - 1, (b[i] as usize) - 1);
    }

    let leaders = dsu
        .build_leader_to_group()
        .keys()
        .copied()
        .collect::<Vec<_>>();

    format!(
        "{}\n{}",
        leaders.len() - 1,
        (0..leaders.len() - 1)
            .map(|i| format!("{} {}", leaders[i] + 1, leaders[i + 1] + 1))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

struct Dsu {
    parent_or_sizes: Vec<i32>,
}

#[allow(dead_code)]
impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent_or_sizes: vec![-1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.parent_or_sizes[a] < 0 {
            return a;
        }

        self.parent_or_sizes[a] = self.find(self.parent_or_sizes[a] as usize) as i32;

        self.parent_or_sizes[a] as usize
    }

    fn union(&mut self, a: usize, b: usize) {
        let a_leader = self.find(a);
        let b_leader = self.find(b);
        if a_leader != b_leader {
            self.parent_or_sizes[a_leader] += self.parent_or_sizes[b_leader];
            self.parent_or_sizes[b_leader] = a_leader as i32;
        }
    }

    fn get_size(&mut self, a: usize) -> usize {
        let leader = self.find(a);

        -self.parent_or_sizes[leader] as usize
    }

    fn build_leader_to_group(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut leader_to_group = HashMap::new();
        for i in 0..self.parent_or_sizes.len() {
            leader_to_group
                .entry(self.find(i))
                .or_insert(Vec::new())
                .push(i);
        }

        leader_to_group
    }
}
