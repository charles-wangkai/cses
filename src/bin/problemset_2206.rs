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
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&p, &queries));
}

fn solve(p: &[i32], queries: &[String]) -> String {
    let mut sum_segment_tree = build_node(
        &(0..p.len()).map(|i| p[i] + (i as i32)).collect::<Vec<_>>(),
        0,
        p.len() - 1,
    );
    let mut diff_segment_tree = build_node(
        &(0..p.len()).map(|i| p[i] - (i as i32)).collect::<Vec<_>>(),
        0,
        p.len() - 1,
    );

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let k: usize = split.next().unwrap().parse().unwrap();
            let x: i32 = split.next().unwrap().parse().unwrap();

            update_segment_tree(k - 1, x + ((k as i32) - 1), &mut sum_segment_tree);
            update_segment_tree(k - 1, x - ((k as i32) - 1), &mut diff_segment_tree);
        } else {
            let k: usize = split.next().unwrap().parse().unwrap();

            result.push(
                (query_segment_tree(k - 1, p.len() - 1, &sum_segment_tree) - ((k as i32) - 1))
                    .min(query_segment_tree(0, k - 1, &diff_segment_tree) + ((k as i32) - 1)),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
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

fn build_node(values: &[i32], begin_index: usize, end_index: usize) -> Box<Node> {
    if begin_index == end_index {
        return Box::new(Node {
            begin_index,
            end_index,
            min_value: values[begin_index],
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
