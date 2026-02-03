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
    let mut seg_tree = SegTree::new(x);

    let mut result = Vec::new();
    for query in queries {
        let mut split = query.split_whitespace();
        let kind: i32 = split.next().unwrap().parse().unwrap();
        if kind == 1 {
            let k: usize = split.next().unwrap().parse().unwrap();
            let u = split.next().unwrap().parse().unwrap();

            seg_tree.update(k - 1, u);
        } else {
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();

            result.push(seg_tree.query(a - 1, b - 1));
        }
    }

    result
        .iter()
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
            node.min_value = values[begin_index];
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

    fn update(&mut self, index: usize, value: i32) {
        Self::update_node(index, value, &mut self.root);
    }

    fn update_node(index: usize, value: i32, node: &mut Node) {
        if node.begin_index <= index && node.end_index >= index {
            if node.begin_index == node.end_index {
                node.min_value = value;
            } else {
                Self::update_node(index, value, node.left.as_mut().unwrap());
                Self::update_node(index, value, node.right.as_mut().unwrap());

                node.pull();
            }
        }
    }

    fn query(&self, begin_index: usize, end_index: usize) -> i32 {
        Self::query_node(begin_index, end_index, &self.root)
    }

    fn query_node(begin_index: usize, end_index: usize, node: &Node) -> i32 {
        if node.begin_index > end_index || node.end_index < begin_index {
            return i32::MAX;
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.min_value;
        }

        Self::query_node(begin_index, end_index, node.left.as_ref().unwrap()).min(Self::query_node(
            begin_index,
            end_index,
            node.right.as_ref().unwrap(),
        ))
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    min_value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize) -> Self {
        Self {
            begin_index,
            end_index,
            min_value: 0,
            left: None,
            right: None,
        }
    }

    fn pull(&mut self) {
        self.min_value = self
            .left
            .as_ref()
            .unwrap()
            .min_value
            .min(self.right.as_ref().unwrap().min_value);
    }
}
