use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut squares = vec![vec![b'\0'; n]; n];
    for square_i in squares.iter_mut() {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        square_i.copy_from_slice(line.trim().as_bytes());
    }
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        queries.push(line);
    }

    println!("{}", solve(&mut squares, &queries));
}

fn solve(squares: &mut [Vec<u8>], queries: &[String]) -> String {
    let n = squares.len();

    let mut fenwick_tree = FenwickTree::new(n, n);
    for x in 1..=n {
        for y in 1..=n {
            if squares[x - 1][y - 1] == b'*' {
                fenwick_tree.add(x, y, 1);
            }
        }
    }

    let mut result = Vec::new();
    for query in queries {
        let parts: Vec<_> = query.split_whitespace().collect();
        if parts[0] == "1" {
            let r: usize = parts[1].parse().unwrap();
            let c = parts[2].parse().unwrap();

            if squares[r - 1][c - 1] == b'*' {
                fenwick_tree.add(r, c, -1);
                squares[r - 1][c - 1] = b'.';
            } else {
                fenwick_tree.add(r, c, 1);
                squares[r - 1][c - 1] = b'*';
            }
        } else {
            let r1: usize = parts[1].parse().unwrap();
            let c1: usize = parts[2].parse().unwrap();
            let r2 = parts[3].parse().unwrap();
            let c2 = parts[4].parse().unwrap();

            result.push(
                fenwick_tree.compute_prefix_sum(r2, c2)
                    - fenwick_tree.compute_prefix_sum(r1 - 1, c2)
                    - fenwick_tree.compute_prefix_sum(r2, c1 - 1)
                    + fenwick_tree.compute_prefix_sum(r1 - 1, c1 - 1),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

struct FenwickTree {
    a: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl FenwickTree {
    fn new(size1: usize, size2: usize) -> Self {
        Self {
            a: vec![vec![0; (1 << (size2.ilog2() + 1)) + 1]; (1 << (size1.ilog2() + 1)) + 1],
        }
    }

    fn add(&mut self, mut x: usize, y: usize, delta: i32) {
        while x < self.a.len() {
            self.add_y(x, y, delta);
            x += ((x as i32) & -(x as i32)) as usize;
        }
    }

    fn add_y(&mut self, x: usize, mut y: usize, delta: i32) {
        while y < self.a.len() {
            self.a[x][y] += delta;
            y += ((y as i32) & -(y as i32)) as usize;
        }
    }

    fn compute_prefix_sum(&self, mut x: usize, y: usize) -> i32 {
        let mut result = 0;
        while x != 0 {
            result += self.compute_prefix_sum_y(x, y);
            x -= ((x as i32) & -(x as i32)) as usize;
        }

        result
    }

    fn compute_prefix_sum_y(&self, x: usize, mut y: usize) -> i32 {
        let mut result = 0;
        while y != 0 {
            result += self.a[x][y];
            y -= ((y as i32) & -(y as i32)) as usize;
        }

        result
    }
}
