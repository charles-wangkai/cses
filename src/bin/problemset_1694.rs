// https://blog.csdn.net/Richard__Luan/article/details/81002097

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
    let mut c = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        c.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b, &c));
}

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32]) -> i64 {
    let mut edges = Vec::new();
    let mut edge_lists = vec![Vec::new(); n];
    for i in 0..a.len() {
        edges.push(Edge {
            to: b[i] - 1,
            capacity: c[i],
        });
        edge_lists[a[i] - 1].push(edges.len() - 1);

        edges.push(Edge {
            to: a[i] - 1,
            capacity: 0,
        });
        edge_lists[b[i] - 1].push(edges.len() - 1);
    }

    dinic(&mut edges, &edge_lists, 0, n - 1)
}

fn dinic(edges: &mut [Edge], edge_lists: &[Vec<usize>], s: usize, t: usize) -> i64 {
    let mut result = 0;
    while let Some(mut levels) = bfs(edges, edge_lists, s, t) {
        loop {
            let minflow = dfs(edges, edge_lists, &mut levels, s, t, i32::MAX);
            if minflow == 0 {
                break;
            }

            result += minflow as i64;
        }
    }

    result
}

fn bfs(edges: &[Edge], edge_lists: &[Vec<usize>], s: usize, t: usize) -> Option<Vec<i32>> {
    let mut levels = vec![-1; edges.len()];
    levels[s] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(s);

    while let Some(head) = queue.pop_front() {
        if head == t {
            return Some(levels);
        }

        for &e in &edge_lists[head] {
            if edges[e].capacity != 0 && levels[edges[e].to] == -1 {
                levels[edges[e].to] = levels[head] + 1;
                queue.push_back(edges[e].to);
            }
        }
    }

    None
}

fn dfs(
    edges: &mut [Edge],
    edge_lists: &[Vec<usize>],
    levels: &mut [i32],
    s: usize,
    t: usize,
    low: i32,
) -> i32 {
    if s == t {
        return low;
    }

    let mut result = 0;
    for &e in &edge_lists[s] {
        if edges[e].capacity != 0 && levels[edges[e].to] == levels[s] + 1 {
            let next = dfs(
                edges,
                edge_lists,
                levels,
                edges[e].to,
                t,
                (low - result).min(edges[e].capacity),
            );
            edges[e].capacity -= next;
            edges[e ^ 1].capacity += next;

            result += next;
            if result == low {
                break;
            }
        }
    }

    if result == 0 {
        levels[s] = -1;
    }

    result
}

struct Edge {
    to: usize,
    capacity: i32,
}
