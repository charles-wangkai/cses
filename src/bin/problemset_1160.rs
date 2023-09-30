use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

const BIT_NUM: usize = 18;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(split.next().unwrap().parse().unwrap());
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

    println!("{}", solve(&t, &a, &b));
}

fn solve(t: &[usize], a: &[usize], b: &[usize]) -> String {
    let n = t.len();

    let mut positions = vec![i32::MAX; n];
    let mut cycles = vec![usize::MAX; n];
    for i in 0..positions.len() {
        if positions[i] == i32::MAX {
            let mut nodes = Vec::new();
            let mut node_to_index = HashMap::new();
            let mut current = i;
            loop {
                if positions[current] != i32::MAX {
                    for j in 0..nodes.len() {
                        positions[nodes[j]] = positions[current] - ((nodes.len() - j) as i32);
                    }

                    break;
                }
                if node_to_index.contains_key(&current) {
                    for j in 0..nodes.len() {
                        positions[nodes[j]] = j as i32;
                    }
                    for j in node_to_index[&current]..nodes.len() {
                        cycles[nodes[j]] = nodes.len() - node_to_index[&current];
                    }

                    break;
                }

                nodes.push(current);
                node_to_index.insert(current, nodes.len() - 1);

                current = t[current] - 1;
            }
        }
    }

    let mut destinations = vec![[usize::MAX; BIT_NUM]; t.len()];
    for i in 0..destinations.len() {
        destinations[i][0] = t[i] - 1;
    }
    for b in 1..BIT_NUM {
        for i in 0..destinations.len() {
            destinations[i][b] = destinations[destinations[i][b - 1]][b - 1];
        }
    }

    (0..a.len())
        .map(|i| {
            if positions[b[i] - 1] >= positions[a[i] - 1]
                && go(
                    &destinations,
                    a[i] - 1,
                    (positions[b[i] - 1] - positions[a[i] - 1]) as usize,
                ) == b[i] - 1
            {
                return positions[b[i] - 1] - positions[a[i] - 1];
            }
            if cycles[b[i] - 1] != usize::MAX
                && positions[b[i] - 1] + (cycles[b[i] - 1] as i32) >= positions[a[i] - 1]
                && go(
                    &destinations,
                    a[i] - 1,
                    (positions[b[i] - 1] + (cycles[b[i] - 1] as i32) - positions[a[i] - 1])
                        as usize,
                ) == b[i] - 1
            {
                return positions[b[i] - 1] + (cycles[b[i] - 1] as i32) - positions[a[i] - 1];
            }

            -1
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn go(destinations: &[[usize; BIT_NUM]], begin_node: usize, step_num: usize) -> usize {
    let mut result = begin_node;
    for j in 0..BIT_NUM {
        if ((step_num >> j) & 1) == 1 {
            result = destinations[result][j];
        }
    }

    result
}
