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
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut k = Vec::new();
    let mut values = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        k.push(split.next().unwrap().parse().unwrap());
        values.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &k, &values));
}

fn solve(x: &[i32], k: &[usize], values: &[i32]) -> String {
    let mut segment_tree = build_node(0, x.len() - 1);
    for (i, &xi) in x.iter().enumerate() {
        update_segment_tree(i, xi, &mut segment_tree);
    }

    let mut result = Vec::new();
    for i in 0..k.len() {
        update_segment_tree(k[i] - 1, values[i], &mut segment_tree);
        result.push(segment_tree.max_subarray_sum);
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
            node.max_subarray_sum = 0.max(value) as i64;
            node.max_prefix_sum = 0.max(value) as i64;
            node.max_suffix_sum = 0.max(value) as i64;
            node.sum = value as i64;
        } else {
            update_segment_tree(index, value, node.left.as_mut().unwrap());
            update_segment_tree(index, value, node.right.as_mut().unwrap());

            let left = node.left.as_ref().unwrap();
            let right = node.right.as_ref().unwrap();
            node.max_subarray_sum = left
                .max_subarray_sum
                .max(right.max_subarray_sum)
                .max(left.max_suffix_sum + right.max_prefix_sum);
            node.max_prefix_sum = left.max_prefix_sum.max(left.sum + right.max_prefix_sum);
            node.max_suffix_sum = right.max_suffix_sum.max(right.sum + left.max_suffix_sum);
            node.sum = left.sum + right.sum;
        }
    }
}

fn build_node(begin_index: usize, end_index: usize) -> Box<Node> {
    if begin_index == end_index {
        return Box::new(Node {
            begin_index,
            end_index,
            max_subarray_sum: 0,
            max_prefix_sum: 0,
            max_suffix_sum: 0,
            sum: 0,
            left: None,
            right: None,
        });
    }

    let middle_index = (begin_index + end_index) / 2;
    let left = build_node(begin_index, middle_index);
    let right = build_node(middle_index + 1, end_index);

    Box::new(Node {
        begin_index,
        end_index,
        max_subarray_sum: 0,
        max_prefix_sum: 0,
        max_suffix_sum: 0,
        sum: 0,
        left: Some(left),
        right: Some(right),
    })
}

struct Node {
    begin_index: usize,
    end_index: usize,
    max_subarray_sum: i64,
    max_prefix_sum: i64,
    max_suffix_sum: i64,
    sum: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
