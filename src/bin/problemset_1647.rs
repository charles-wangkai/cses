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
    let sparse_table = SparseTable::new(x, i32::min);

    (0..a.len())
        .map(|i| sparse_table.query(a[i] - 1, b[i] - 1))
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct SparseTable {
    st: Vec<Vec<i32>>,
    operator: fn(i32, i32) -> i32,
}

#[allow(dead_code)]
impl SparseTable {
    fn new(values: &[i32], operator: fn(i32, i32) -> i32) -> Self {
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

    fn query(&self, begin_index: usize, end_index: usize) -> i32 {
        let exponent = (end_index - begin_index + 1).ilog2() as usize;

        (self.operator)(
            self.st[begin_index][exponent],
            self.st[end_index + 1 - (1 << exponent)][exponent],
        )
    }
}
