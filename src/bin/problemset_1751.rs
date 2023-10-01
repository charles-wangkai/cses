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
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&t));
}

fn solve(t: &[usize]) -> String {
    let mut step_nums = vec![usize::MAX; t.len()];
    for i in 0..step_nums.len() {
        if step_nums[i] == usize::MAX {
            let mut nodes = Vec::new();
            let mut node_to_index = HashMap::new();
            let mut current = i;
            loop {
                if step_nums[current] != usize::MAX {
                    for j in 0..nodes.len() {
                        step_nums[nodes[j]] = step_nums[current] + (nodes.len() - j);
                    }

                    break;
                }
                if node_to_index.contains_key(&current) {
                    for j in 0..node_to_index[&current] {
                        step_nums[nodes[j]] = nodes.len() - j;
                    }
                    for j in node_to_index[&current]..nodes.len() {
                        step_nums[nodes[j]] = nodes.len() - node_to_index[&current];
                    }

                    break;
                }

                nodes.push(current);
                node_to_index.insert(current, nodes.len() - 1);

                current = t[current] - 1;
            }
        }
    }

    step_nums
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
