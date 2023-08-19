use std::{
    collections::VecDeque,
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

fn solve(n: usize, a: &[usize], b: &[usize]) -> String {
    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let mut nexts = vec![usize::MAX; n];
    nexts[n - 1] = n - 1;

    let mut queue = VecDeque::new();
    queue.push_back(n - 1);

    while let Some(head) = queue.pop_front() {
        for &adj in &adj_vecs[head] {
            if nexts[adj] == usize::MAX {
                nexts[adj] = head;
                queue.push_back(adj);
            }
        }
    }

    if nexts[0] == usize::MAX {
        return String::from("IMPOSSIBLE");
    }

    let mut node = 0;
    let mut route = vec![node + 1];
    while node != n - 1 {
        node = nexts[node];
        route.push(node + 1);
    }

    format!(
        "{}\n{}",
        route.len(),
        route
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
