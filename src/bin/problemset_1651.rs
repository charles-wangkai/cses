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
    let mut lazy_seg_tree = LazySegTree::new(x.len());
    for i in 0..x.len() {
        lazy_seg_tree.update(i, i, x[i]);
    }

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();
            let u = split.next().unwrap().parse().unwrap();

            lazy_seg_tree.update(a - 1, b - 1, u);
        } else {
            let k: usize = split.next().unwrap().parse().unwrap();

            result.push(lazy_seg_tree.query(k - 1, k - 1));
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
    fn new(size: usize) -> Self {
        Self {
            root: Self::build_node(0, size - 1),
        }
    }

    fn build_node(begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index, 0);

        if begin_index != end_index {
            let middle_index = (begin_index + end_index) / 2;
            node.left = Some(Box::new(Self::build_node(begin_index, middle_index)));
            node.right = Some(Box::new(Self::build_node(middle_index + 1, end_index)));
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
            return node.delta;
        }

        node.push_down();

        Self::query_node(begin_index, end_index, node.left.as_mut().unwrap())
            + Self::query_node(begin_index, end_index, node.right.as_mut().unwrap())
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    delta: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize, delta: i64) -> Self {
        Self {
            begin_index,
            end_index,
            delta,
            left: None,
            right: None,
        }
    }

    fn push_down(&mut self) {
        if self.delta != 0 {
            self.left.as_mut().unwrap().apply(self.delta);
            self.right.as_mut().unwrap().apply(self.delta);

            self.delta = 0;
        }
    }

    fn apply(&mut self, d: i64) {
        self.delta += d;
    }
}
