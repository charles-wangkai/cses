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
            let x = split.next().unwrap().parse().unwrap();

            lazy_seg_tree.update_delta(a - 1, b - 1, x);
        } else if kind == 2 {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();
            let x = split.next().unwrap().parse().unwrap();

            lazy_seg_tree.update_constant(a - 1, b - 1, x);
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
        let mut node = Node::new(begin_index, end_index, None, 0);

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

    fn update_constant(&mut self, begin_index: usize, end_index: usize, constant: i32) {
        Self::update_node(begin_index, end_index, Some(constant), 0, &mut self.root);
    }

    fn update_delta(&mut self, begin_index: usize, end_index: usize, delta: i32) {
        Self::update_node(begin_index, end_index, None, delta, &mut self.root);
    }

    fn update_node(
        begin_index: usize,
        end_index: usize,
        constant: Option<i32>,
        delta: i32,
        node: &mut Node,
    ) {
        if !(node.begin_index > end_index || node.end_index < begin_index) {
            if node.begin_index >= begin_index && node.end_index <= end_index {
                node.apply(constant, delta as i64);
            } else {
                node.push_down();

                Self::update_node(
                    begin_index,
                    end_index,
                    constant,
                    delta,
                    node.left.as_mut().unwrap(),
                );
                Self::update_node(
                    begin_index,
                    end_index,
                    constant,
                    delta,
                    node.right.as_mut().unwrap(),
                );

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
    constant: Option<i32>,
    delta: i64,
    sum: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize, constant: Option<i32>, delta: i64) -> Self {
        Self {
            begin_index,
            end_index,
            constant,
            delta,
            sum: 0,
            left: None,
            right: None,
        }
    }

    fn get_computed_sum(&self) -> i64 {
        (match self.constant {
            Some(c) => (c as i64) * ((self.end_index - self.begin_index + 1) as i64),
            None => self.sum,
        }) + self.delta * ((self.end_index - self.begin_index + 1) as i64)
    }

    fn push_down(&mut self) {
        self.left.as_mut().unwrap().apply(self.constant, self.delta);
        self.right
            .as_mut()
            .unwrap()
            .apply(self.constant, self.delta);

        self.constant = None;
        self.delta = 0;
    }

    fn apply(&mut self, constant: Option<i32>, delta: i64) {
        match constant {
            Some(_) => {
                self.constant = constant;
                self.delta = delta;
            }
            None => self.delta += delta,
        };
    }

    fn pull(&mut self) {
        self.sum = self.left.as_ref().unwrap().get_computed_sum()
            + self.right.as_ref().unwrap().get_computed_sum();
    }
}
