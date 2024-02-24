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

    println!("{}", solve(&mut x, &queries));
}

fn solve(x: &mut [i32], queries: &[String]) -> String {
    let mut prefix_sums = vec![0; x.len()];
    for i in 0..prefix_sums.len() {
        prefix_sums[i] = (if i == 0 { 0 } else { prefix_sums[i - 1] }) + (x[i] as i64);
    }

    let mut segment_tree = build_node(&prefix_sums, 0, prefix_sums.len() - 1);

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let k: usize = split.next().unwrap().parse().unwrap();
            let u = split.next().unwrap().parse().unwrap();

            update_segment_tree(
                k - 1,
                prefix_sums.len() - 1,
                (u - x[k - 1]) as i64,
                &mut segment_tree,
            );
            x[k - 1] = u;
        } else {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            result.push(0.max(
                query_segment_tree(a - 1, b - 1, &mut segment_tree)
                    - (if a == 1 {
                        0
                    } else {
                        query_segment_tree(a - 2, a - 2, &mut segment_tree)
                    }),
            ));
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn query_segment_tree(begin_index: usize, end_index: usize, node: &mut Box<Node>) -> i64 {
    if node.begin_index > end_index || node.end_index < begin_index {
        return i64::MIN;
    }

    if node.begin_index >= begin_index && node.end_index <= end_index {
        return node.delta + node.max_value;
    }

    if node.delta != 0 {
        node.left.as_mut().unwrap().delta += node.delta;
        node.right.as_mut().unwrap().delta += node.delta;
        node.max_value += node.delta;
        node.delta = 0;
    }

    query_segment_tree(begin_index, end_index, node.left.as_mut().unwrap()).max(query_segment_tree(
        begin_index,
        end_index,
        node.right.as_mut().unwrap(),
    ))
}

fn update_segment_tree(begin_index: usize, end_index: usize, delta: i64, node: &mut Box<Node>) {
    if !(node.begin_index > end_index || node.end_index < begin_index) {
        if node.begin_index >= begin_index && node.end_index <= end_index {
            node.delta += delta;
        } else {
            if node.delta != 0 {
                node.left.as_mut().unwrap().delta += node.delta;
                node.right.as_mut().unwrap().delta += node.delta;
                node.delta = 0;
            }

            update_segment_tree(begin_index, end_index, delta, node.left.as_mut().unwrap());
            update_segment_tree(begin_index, end_index, delta, node.right.as_mut().unwrap());

            node.max_value = (node.left.as_ref().unwrap().delta
                + node.left.as_ref().unwrap().max_value)
                .max(node.right.as_ref().unwrap().delta + node.right.as_ref().unwrap().max_value);
        }
    }
}

fn build_node(values: &[i64], begin_index: usize, end_index: usize) -> Box<Node> {
    if begin_index == end_index {
        return Box::new(Node {
            begin_index,
            end_index,
            delta: 0,
            max_value: values[begin_index],
            left: None,
            right: None,
        });
    }

    let middle_index = (begin_index + end_index) / 2;
    let left = build_node(values, begin_index, middle_index);
    let right = build_node(values, middle_index + 1, end_index);

    Box::new(Node {
        begin_index,
        end_index,
        delta: 0,
        max_value: left.max_value.max(right.max_value),
        left: Some(left),
        right: Some(right),
    })
}

struct Node {
    begin_index: usize,
    end_index: usize,
    delta: i64,
    max_value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
