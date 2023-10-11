// https://en.wikipedia.org/wiki/De_Bruijn_sequence
// https://cp-algorithms.com/graph/euler_path.html

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

    println!("{}", solve(n));
}

fn solve(n: i32) -> String {
    if n == 1 {
        return String::from("01");
    }

    let mut node_to_edges = HashMap::new();
    for mask in 0..1 << (n - 1) {
        node_to_edges.insert(
            (0..n - 1)
                .rev()
                .map(|i| (mask >> i) & 1)
                .map(|x| x.to_string())
                .collect::<String>(),
            HashMap::new(),
        );
    }
    for node in node_to_edges.keys().cloned().collect::<Vec<_>>() {
        for b in ['0', '1'] {
            node_to_edges
                .get_mut(&node)
                .unwrap()
                .insert(b, build_next_node(&node, b));
        }
    }

    let mut route = Vec::new();
    let begin_node = node_to_edges.keys().next().cloned().unwrap();
    find_euler_path(&mut route, &mut node_to_edges, &begin_node);
    route.reverse();

    format!(
        "{}{}",
        route[0],
        route[1..]
            .iter()
            .map(|node| node.chars().last().unwrap().to_string())
            .collect::<String>()
    )
}

fn build_next_node(node: &str, b: char) -> String {
    String::from(&format!("{}{}", node, b)[1..])
}

fn find_euler_path(
    route: &mut Vec<String>,
    node_to_edges: &mut HashMap<String, HashMap<char, String>>,
    node: &str,
) {
    while let Some((&b, next_node)) = node_to_edges[node].iter().next() {
        let next_node = next_node.clone();

        node_to_edges.get_mut(node).unwrap().remove(&b);

        find_euler_path(route, node_to_edges, &next_node);
    }

    route.push(String::from(node));
}
