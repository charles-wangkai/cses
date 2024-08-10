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
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(split.next().unwrap().parse().unwrap());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&t, &queries));
}

fn solve(t: &[i32], queries: &[String]) -> String {
    let mut segment_tree = build_node(t, 0, t.len() - 1);

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            update_segment_tree(a - 1, b - 1, &mut segment_tree);
        } else {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            result.push(query_segment_tree(a - 1, b - 1, &mut segment_tree));
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
        return 0;
    }

    if node.begin_index >= begin_index && node.end_index <= end_index {
        return node.computed_sum();
    }

    node.push_down();

    query_segment_tree(begin_index, end_index, node.left.as_mut().unwrap())
        + query_segment_tree(begin_index, end_index, node.right.as_mut().unwrap())
}

fn update_segment_tree(begin_index: usize, end_index: usize, node: &mut Box<Node>) {
    if !(node.begin_index > end_index || node.end_index < begin_index) {
        if node.begin_index >= begin_index && node.end_index <= end_index {
            node.accept_update((node.begin_index - begin_index + 1) as i64, 1);
        } else {
            node.push_down();

            update_segment_tree(begin_index, end_index, node.left.as_mut().unwrap());
            update_segment_tree(begin_index, end_index, node.right.as_mut().unwrap());

            node.sum = node.left.as_ref().unwrap().computed_sum()
                + node.right.as_ref().unwrap().computed_sum();
        }
    }
}

fn build_node(t: &[i32], begin_index: usize, end_index: usize) -> Box<Node> {
    if begin_index == end_index {
        return Box::new(Node {
            begin_index,
            end_index,
            begin_delta: 0,
            delta_diff: 0,
            sum: t[begin_index] as i64,
            left: None,
            right: None,
        });
    }

    let middle_index = (begin_index + end_index) / 2;
    let left = build_node(t, begin_index, middle_index);
    let right = build_node(t, middle_index + 1, end_index);

    Box::new(Node {
        begin_index,
        end_index,
        begin_delta: 0,
        delta_diff: 0,
        sum: left.sum + right.sum,
        left: Some(left),
        right: Some(right),
    })
}

struct Node {
    begin_index: usize,
    end_index: usize,
    begin_delta: i64,
    delta_diff: i32,
    sum: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn length(&self) -> usize {
        self.end_index - self.begin_index + 1
    }

    fn computed_sum(&self) -> i64 {
        self.sum
            + ((self.length() as i64) * self.begin_delta
                + (self.length() as i64) * ((self.length() as i64) - 1) / 2
                    * (self.delta_diff as i64))
    }

    fn push_down(&mut self) {
        self.left
            .as_mut()
            .unwrap()
            .accept_update(self.begin_delta, self.delta_diff);
        self.right.as_mut().unwrap().accept_update(
            self.begin_delta
                + (self.left.as_ref().unwrap().length() as i64) * (self.delta_diff as i64),
            self.delta_diff,
        );

        self.sum = self.computed_sum();

        self.begin_delta = 0;
        self.delta_diff = 0;
    }

    fn accept_update(&mut self, begin_delta: i64, delta_diff: i32) {
        self.begin_delta += begin_delta;
        self.delta_diff += delta_diff;
    }
}
