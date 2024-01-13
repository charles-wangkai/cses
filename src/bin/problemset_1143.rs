use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut h = Vec::new();
    for _ in 0..n {
        h.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut r = Vec::new();
    for _ in 0..m {
        r.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&mut h, &r));
}

fn solve(h: &mut [i32], r: &[i32]) -> String {
    let mut segment_tree = build_node(h, 0, h.len() - 1);

    let mut result = Vec::new();
    for &ri in r {
        let index = find_index(h.len(), &segment_tree, ri);
        if index == usize::MAX {
            result.push(0);
        } else {
            h[index] -= ri;
            update_segment_tree(index, h[index], &mut segment_tree);

            result.push(index + 1);
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn find_index(n: usize, segment_tree: &Node, target: i32) -> usize {
    let mut result = usize::MAX;
    let mut lower = 0;
    let mut upper = (n as i32) - 1;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if query_segment_tree(0, middle as usize, segment_tree) >= target {
            result = middle as usize;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
}

fn query_segment_tree(begin_index: usize, end_index: usize, node: &Node) -> i32 {
    if node.begin_index > end_index || node.end_index < begin_index {
        return i32::MIN;
    }
    if node.begin_index >= begin_index && node.end_index <= end_index {
        return node.max_value;
    }

    query_segment_tree(begin_index, end_index, node.left.as_ref().unwrap()).max(query_segment_tree(
        begin_index,
        end_index,
        node.right.as_ref().unwrap(),
    ))
}

fn update_segment_tree(index: usize, value: i32, node: &mut Box<Node>) {
    if node.begin_index <= index && node.end_index >= index {
        if node.begin_index == node.end_index {
            node.max_value = value;
        } else {
            update_segment_tree(index, value, node.left.as_mut().unwrap());
            update_segment_tree(index, value, node.right.as_mut().unwrap());

            node.max_value = node
                .left
                .as_ref()
                .unwrap()
                .max_value
                .max(node.right.as_ref().unwrap().max_value);
        }
    }
}

fn build_node(h: &[i32], begin_index: usize, end_index: usize) -> Box<Node> {
    if begin_index == end_index {
        return Box::new(Node {
            begin_index,
            end_index,
            max_value: h[begin_index],
            left: None,
            right: None,
        });
    }

    let middle_index = (begin_index + end_index) / 2;
    let left = build_node(h, begin_index, middle_index);
    let right = build_node(h, middle_index + 1, end_index);

    Box::new(Node {
        begin_index,
        end_index,
        max_value: left.max_value.max(right.max_value),
        left: Some(left),
        right: Some(right),
    })
}

struct Node {
    begin_index: usize,
    end_index: usize,
    max_value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
