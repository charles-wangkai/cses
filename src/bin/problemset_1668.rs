use std::io::{stdin, BufRead, BufReader};

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

    let mut teams = vec![0; n];
    for i in 0..teams.len() {
        if teams[i] == 0 && !assign(&adj_vecs, &mut teams, i, 1) {
            return String::from("IMPOSSIBLE");
        }
    }

    teams
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn assign(adj_vecs: &[Vec<usize>], teams: &mut [i32], node: usize, team: i32) -> bool {
    if teams[node] != 0 {
        return teams[node] == team;
    }

    teams[node] = team;
    for &adj in &adj_vecs[node] {
        if !assign(adj_vecs, teams, adj, 3 - team) {
            return false;
        }
    }

    true
}
