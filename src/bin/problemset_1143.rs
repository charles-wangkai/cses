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
    let mut seg_tree = SegTree::new(h);

    let mut result = Vec::new();
    for &ri in r {
        let index = find_index(h.len(), &seg_tree, ri);
        if index == usize::MAX {
            result.push(0);
        } else {
            h[index] -= ri;
            seg_tree.update(index, h[index]);

            result.push(index + 1);
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn find_index(n: usize, seg_tree: &SegTree, target: i32) -> usize {
    let mut result = usize::MAX;
    let mut lower = 0;
    let mut upper = (n as i32) - 1;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if seg_tree.query(0, middle as usize) >= target {
            result = middle as usize;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
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

    fn update(&mut self, index: usize, value: i32) {
        Self::update_node(index, value, &mut self.root);
    }

    fn update_node(index: usize, value: i32, node: &mut Node) {
        if node.begin_index <= index && node.end_index >= index {
            if node.begin_index == node.end_index {
                node.max_value = value;
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
            return i32::MIN;
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.max_value;
        }

        Self::query_node(begin_index, end_index, node.left.as_ref().unwrap()).max(Self::query_node(
            begin_index,
            end_index,
            node.right.as_ref().unwrap(),
        ))
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    max_value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize) -> Self {
        Self {
            begin_index,
            end_index,
            max_value: 0,
            left: None,
            right: None,
        }
    }

    fn pull(&mut self) {
        self.max_value = self
            .left
            .as_ref()
            .unwrap()
            .max_value
            .max(self.right.as_ref().unwrap().max_value);
    }
}
