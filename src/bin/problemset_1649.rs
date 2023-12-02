use std::io::{stdin, BufRead, BufReader};

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
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&x, &queries));
}

fn solve(x: &[i32], queries: &[String]) -> String {
    let mut segment_tree = build_node(x, 0, x.len() - 1);

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let k: usize = split.next().unwrap().parse().unwrap();
            let u = split.next().unwrap().parse().unwrap();

            update_segment_tree(k - 1, u, &mut segment_tree);
        } else {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            result.push(query_segment_tree(a - 1, b - 1, &segment_tree));
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn update_segment_tree(index: usize, value: i32, node: &mut Box<Node>) {
    if node.begin_index <= index && node.end_index >= index {
        if node.begin_index == node.end_index {
            node.min_value = value;
        } else {
            update_segment_tree(index, value, node.left.as_mut().unwrap());
            update_segment_tree(index, value, node.right.as_mut().unwrap());

            node.min_value = node
                .left
                .as_ref()
                .unwrap()
                .min_value
                .min(node.right.as_ref().unwrap().min_value);
        }
    }
}

fn query_segment_tree(begin_index: usize, end_index: usize, node: &Node) -> i32 {
    if node.begin_index > end_index || node.end_index < begin_index {
        return i32::MAX;
    }
    if node.begin_index >= begin_index && node.end_index <= end_index {
        return node.min_value;
    }

    query_segment_tree(begin_index, end_index, node.left.as_ref().unwrap()).min(query_segment_tree(
        begin_index,
        end_index,
        node.right.as_ref().unwrap(),
    ))
}

fn build_node(x: &[i32], begin_index: usize, end_index: usize) -> Box<Node> {
    if begin_index == end_index {
        return Box::new(Node {
            begin_index,
            end_index,
            min_value: x[begin_index],
            left: None,
            right: None,
        });
    }

    let middle_index = (begin_index + end_index) / 2;
    let left = build_node(x, begin_index, middle_index);
    let right = build_node(x, middle_index + 1, end_index);

    Box::new(Node {
        begin_index,
        end_index,
        min_value: left.min_value.min(right.min_value),
        left: Some(left),
        right: Some(right),
    })
}

struct Node {
    begin_index: usize,
    end_index: usize,
    min_value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
