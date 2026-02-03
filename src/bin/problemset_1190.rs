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
    let mut seg_tree = SegTree::new(x.len());
    for i in 0..x.len() {
        seg_tree.update(i, x[i]);
    }

    let mut result = Vec::new();
    for i in 0..k.len() {
        seg_tree.update(k[i] - 1, values[i]);
        result.push(seg_tree.root.max_subarray_sum);
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
    fn new(size: usize) -> Self {
        Self {
            root: Self::build_node(0, size - 1),
        }
    }

    fn build_node(begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index);

        if begin_index == end_index {
            node.max_subarray_sum = 0;
            node.max_prefix_sum = 0;
            node.max_suffix_sum = 0;
            node.sum = 0;
        } else {
            let middle_index = (begin_index + end_index) / 2;
            node.left = Some(Box::new(Self::build_node(begin_index, middle_index)));
            node.right = Some(Box::new(Self::build_node(middle_index + 1, end_index)));

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
                node.max_subarray_sum = 0.max(value) as i64;
                node.max_prefix_sum = 0.max(value) as i64;
                node.max_suffix_sum = 0.max(value) as i64;
                node.sum = value as i64;
            } else {
                Self::update_node(index, value, node.left.as_mut().unwrap());
                Self::update_node(index, value, node.right.as_mut().unwrap());

                node.pull();
            }
        }
    }
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

impl Node {
    fn new(begin_index: usize, end_index: usize) -> Self {
        Self {
            begin_index,
            end_index,
            max_subarray_sum: 0,
            max_prefix_sum: 0,
            max_suffix_sum: 0,
            sum: 0,
            left: None,
            right: None,
        }
    }

    fn pull(&mut self) {
        let left = self.left.as_ref().unwrap();
        let right = self.right.as_ref().unwrap();

        self.max_subarray_sum = left
            .max_subarray_sum
            .max(right.max_subarray_sum)
            .max(left.max_suffix_sum + right.max_prefix_sum);
        self.max_prefix_sum = left.max_prefix_sum.max(left.sum + right.max_prefix_sum);
        self.max_suffix_sum = right.max_suffix_sum.max(right.sum + left.max_suffix_sum);
        self.sum = left.sum + right.sum;
    }
}
