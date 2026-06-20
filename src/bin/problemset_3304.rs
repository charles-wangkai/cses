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
    let mut h = Vec::new();
    for _ in 0..n {
        h.push(split.next().unwrap().parse().unwrap());
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

    println!("{}", solve(&h, &a, &b));
}

fn solve(h: &[i32], a: &[usize], b: &[usize]) -> String {
    let n = h.len();

    let mut right_seen_building_nums = vec![0; n];
    let mut stack = Vec::new();
    for i in (0..right_seen_building_nums.len()).rev() {
        while !stack.is_empty() && h[stack.last().copied().unwrap()] <= h[i] {
            stack.pop();
        }

        right_seen_building_nums[i] = 1
            + (if stack.is_empty() {
                0
            } else {
                right_seen_building_nums[stack.last().copied().unwrap()]
            });

        stack.push(i);
    }

    let operator = |a: usize, b: usize| -> usize {
        if h[a] >= h[b] {
            a
        } else {
            b
        }
    };

    let sparse_table = SparseTable::new(&((0..n).collect::<Vec<_>>()), operator);

    (0..a.len())
        .map(|i| {
            right_seen_building_nums[a[i] - 1]
                - right_seen_building_nums[sparse_table.query(a[i] - 1, b[i] - 1)]
                + 1
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct SparseTable<F> {
    st: Vec<Vec<usize>>,
    operator: F,
}

#[allow(dead_code)]
impl<F> SparseTable<F>
where
    F: Fn(usize, usize) -> usize,
{
    fn new(values: &[usize], operator: F) -> Self {
        let size = (values.len().ilog2() as usize) + 1;
        let mut st = vec![vec![0; size]; values.len()];
        for i in 0..values.len() {
            st[i][0] = values[i];
        }
        for exponent in 1..size {
            for i in 0..values.len() {
                if i + (1 << exponent) <= values.len() {
                    st[i][exponent] = operator(
                        st[i][exponent - 1],
                        st[i + (1 << (exponent - 1))][exponent - 1],
                    );
                }
            }
        }

        Self { st, operator }
    }

    fn query(&self, begin_index: usize, end_index: usize) -> usize {
        let exponent = (end_index - begin_index + 1).ilog2() as usize;

        (self.operator)(
            self.st[begin_index][exponent],
            self.st[end_index + 1 - (1 << exponent)][exponent],
        )
    }
}
