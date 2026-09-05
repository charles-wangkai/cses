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
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &a, &b));
}

fn solve(x: &[i32], a: &[usize], b: &[usize]) -> String {
    let seg_tree = SegTree::new(x);

    (0..a.len())
        .map(|i| seg_tree.query(a[i] - 1, b[i] - 1))
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct SegTree {
    root: Node,
}

#[allow(dead_code)]
impl SegTree {
    fn new(values: &[i32]) -> Self {
        Self {
            root: Self::build_node(values, 0, values.len() - 1),
        }
    }

    fn build_node(values: &[i32], begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index);

        if begin_index == end_index {
            node.outcome.max_sum = 0.max(values[begin_index] as i64);
            node.outcome.left_max_sum = 0.max(values[begin_index] as i64);
            node.outcome.right_max_sum = 0.max(values[begin_index] as i64);
            node.outcome.total = values[begin_index] as i64;
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

    fn query(&self, begin_index: usize, end_index: usize) -> i64 {
        Self::query_node(begin_index, end_index, &self.root).max_sum
    }

    fn query_node(begin_index: usize, end_index: usize, node: &Node) -> Outcome {
        if node.begin_index > end_index || node.end_index < begin_index {
            return Outcome::new();
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.outcome.clone();
        }

        Outcome::merge(
            &Self::query_node(begin_index, end_index, node.left.as_ref().unwrap()),
            &Self::query_node(begin_index, end_index, node.right.as_ref().unwrap()),
        )
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    outcome: Outcome,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize) -> Self {
        Self {
            begin_index,
            end_index,
            outcome: Outcome::new(),
            left: None,
            right: None,
        }
    }

    fn pull(&mut self) {
        self.outcome = Outcome::merge(
            &self.left.as_ref().unwrap().outcome,
            &self.right.as_ref().unwrap().outcome,
        );
    }
}

#[derive(Clone)]
struct Outcome {
    max_sum: i64,
    left_max_sum: i64,
    right_max_sum: i64,
    total: i64,
}

impl Outcome {
    fn new() -> Self {
        Self {
            max_sum: 0,
            left_max_sum: 0,
            right_max_sum: 0,
            total: 0,
        }
    }

    fn merge(left: &Outcome, right: &Outcome) -> Self {
        Self {
            max_sum: 0
                .max(left.max_sum.max(right.max_sum))
                .max(left.right_max_sum + right.left_max_sum),
            left_max_sum: left.left_max_sum.max(left.total + right.left_max_sum),
            right_max_sum: right.right_max_sum.max(right.total + left.right_max_sum),
            total: left.total + right.total,
        }
    }
}
