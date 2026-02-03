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

    let mut lazy_seg_tree = LazySegTree::new(&prefix_sums);

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let k: usize = split.next().unwrap().parse().unwrap();
            let u = split.next().unwrap().parse().unwrap();

            lazy_seg_tree.update(k - 1, prefix_sums.len() - 1, u - x[k - 1]);
            x[k - 1] = u;
        } else {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            result.push(0.max(
                lazy_seg_tree.query(a - 1, b - 1)
                    - (if a == 1 {
                        0
                    } else {
                        lazy_seg_tree.query(a - 2, a - 2)
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

struct LazySegTree {
    root: Node,
}

#[allow(dead_code)]
impl LazySegTree {
    fn new(values: &[i64]) -> Self {
        Self {
            root: Self::build_node(values, 0, values.len() - 1),
        }
    }

    fn build_node(values: &[i64], begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index, 0);

        if begin_index == end_index {
            node.max_value = values[begin_index];
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

    fn update(&mut self, begin_index: usize, end_index: usize, delta: i32) {
        Self::update_node(begin_index, end_index, delta, &mut self.root);
    }

    fn update_node(begin_index: usize, end_index: usize, delta: i32, node: &mut Node) {
        if !(node.begin_index > end_index || node.end_index < begin_index) {
            if node.begin_index >= begin_index && node.end_index <= end_index {
                node.apply(delta as i64);
            } else {
                node.push_down();

                Self::update_node(begin_index, end_index, delta, node.left.as_mut().unwrap());
                Self::update_node(begin_index, end_index, delta, node.right.as_mut().unwrap());

                node.pull();
            }
        }
    }

    fn query(&mut self, begin_index: usize, end_index: usize) -> i64 {
        Self::query_node(begin_index, end_index, &mut self.root)
    }

    fn query_node(begin_index: usize, end_index: usize, node: &mut Node) -> i64 {
        if node.begin_index > end_index || node.end_index < begin_index {
            return i64::MIN;
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.get_computed_max_value();
        }

        node.push_down();

        node.pull();

        Self::query_node(begin_index, end_index, node.left.as_mut().unwrap()).max(Self::query_node(
            begin_index,
            end_index,
            node.right.as_mut().unwrap(),
        ))
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    delta: i64,
    max_value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize, delta: i64) -> Self {
        Self {
            begin_index,
            end_index,
            delta,
            max_value: 0,
            left: None,
            right: None,
        }
    }

    fn get_computed_max_value(&self) -> i64 {
        self.max_value + self.delta
    }

    fn push_down(&mut self) {
        self.left.as_mut().unwrap().apply(self.delta);
        self.right.as_mut().unwrap().apply(self.delta);

        self.delta = 0;
    }

    fn apply(&mut self, d: i64) {
        self.delta += d;
    }

    fn pull(&mut self) {
        self.max_value = self
            .left
            .as_ref()
            .unwrap()
            .get_computed_max_value()
            .max(self.right.as_ref().unwrap().get_computed_max_value());
    }
}
