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
    let mut lazy_seg_tree = LazySegTree::new(t);

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            lazy_seg_tree.update(a - 1, b - 1);
        } else {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            result.push(lazy_seg_tree.query(a - 1, b - 1));
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct LazySegTree {
    root: Node,
}

#[allow(dead_code)]
impl LazySegTree {
    fn new(values: &[i32]) -> Self {
        Self {
            root: Self::build_node(values, 0, values.len() - 1),
        }
    }

    fn build_node(values: &[i32], begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index, 0, 0);

        if begin_index == end_index {
            node.sum = values[begin_index] as i64;
        } else {
            let middle_index = (begin_index + end_index) / 2;
            node.left = Some(Box::new(Self::build_node(
                values,
                begin_index,
                middle_index,
            )));
            node.right = Some(Box::new(Self::build_node(
                values,
                middle_index + 1,
                end_index,
            )));

            node.pull();
        }

        node
    }

    fn update(&mut self, begin_index: usize, end_index: usize) {
        Self::update_node(begin_index, end_index, &mut self.root);
    }

    fn update_node(begin_index: usize, end_index: usize, node: &mut Node) {
        if !(node.begin_index > end_index || node.end_index < begin_index) {
            if node.begin_index >= begin_index && node.end_index <= end_index {
                node.apply((node.begin_index - begin_index + 1) as i64, 1);
            } else {
                node.push_down();

                Self::update_node(begin_index, end_index, node.left.as_mut().unwrap());
                Self::update_node(begin_index, end_index, node.right.as_mut().unwrap());

                node.pull();
            }
        }
    }

    fn query(&mut self, begin_index: usize, end_index: usize) -> i64 {
        Self::query_node(begin_index, end_index, &mut self.root)
    }

    fn query_node(begin_index: usize, end_index: usize, node: &mut Node) -> i64 {
        if node.begin_index > end_index || node.end_index < begin_index {
            return 0;
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.get_computed_sum();
        }

        node.push_down();

        node.pull();

        Self::query_node(begin_index, end_index, node.left.as_mut().unwrap())
            + Self::query_node(begin_index, end_index, node.right.as_mut().unwrap())
    }
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
    fn new(begin_index: usize, end_index: usize, begin_delta: i64, delta_diff: i32) -> Self {
        Self {
            begin_index,
            end_index,
            begin_delta,
            delta_diff,
            sum: 0,
            left: None,
            right: None,
        }
    }

    fn length(&self) -> usize {
        self.end_index - self.begin_index + 1
    }

    fn get_computed_sum(&self) -> i64 {
        self.sum
            + ((self.length() as i64) * self.begin_delta
                + (self.length() as i64) * ((self.length() as i64) - 1) / 2
                    * (self.delta_diff as i64))
    }

    fn push_down(&mut self) {
        self.left
            .as_mut()
            .unwrap()
            .apply(self.begin_delta, self.delta_diff);
        self.right.as_mut().unwrap().apply(
            self.begin_delta
                + (self.left.as_ref().unwrap().length() as i64) * (self.delta_diff as i64),
            self.delta_diff,
        );

        self.begin_delta = 0;
        self.delta_diff = 0;
    }

    fn apply(&mut self, begin_delta: i64, delta_diff: i32) {
        self.begin_delta += begin_delta;
        self.delta_diff += delta_diff;
    }

    fn pull(&mut self) {
        self.sum = self.left.as_ref().unwrap().get_computed_sum()
            + self.right.as_ref().unwrap().get_computed_sum();
    }
}
