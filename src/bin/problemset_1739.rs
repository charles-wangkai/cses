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

    let mut binary_indexed_tree =
        vec![vec![0; (1 << (n.ilog2() + 1)) + 1]; (1 << (n.ilog2() + 1)) + 1];
    for x in 1..=n {
        for y in 1..=n {
            if squares[x - 1][y - 1] == b'*' {
                update(&mut binary_indexed_tree, x, y, 1);
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
                update(&mut binary_indexed_tree, r, c, -1);
                squares[r - 1][c - 1] = b'.';
            } else {
                update(&mut binary_indexed_tree, r, c, 1);
                squares[r - 1][c - 1] = b'*';
            }
        } else {
            let r1: usize = parts[1].parse().unwrap();
            let c1: usize = parts[2].parse().unwrap();
            let r2 = parts[3].parse().unwrap();
            let c2 = parts[4].parse().unwrap();

            result.push(
                compute_prefix_sum(&binary_indexed_tree, r2, c2)
                    - compute_prefix_sum(&binary_indexed_tree, r1 - 1, c2)
                    - compute_prefix_sum(&binary_indexed_tree, r2, c1 - 1)
                    + compute_prefix_sum(&binary_indexed_tree, r1 - 1, c1 - 1),
            );
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn compute_prefix_sum(binary_indexed_tree: &[Vec<i32>], mut x: usize, y: usize) -> i32 {
    let mut result = 0;
    while x != 0 {
        result += compute_prefix_sum_y(binary_indexed_tree, x, y);
        x -= ((x as i32) & -(x as i32)) as usize;
    }

    result
}

fn compute_prefix_sum_y(binary_indexed_tree: &[Vec<i32>], x: usize, mut y: usize) -> i32 {
    let mut result = 0;
    while y != 0 {
        result += binary_indexed_tree[x][y];
        y -= ((y as i32) & -(y as i32)) as usize;
    }

    result
}

fn update(binary_indexed_tree: &mut [Vec<i32>], mut x: usize, y: usize, delta: i32) {
    while x < binary_indexed_tree.len() {
        update_y(binary_indexed_tree, x, y, delta);
        x += ((x as i32) & -(x as i32)) as usize;
    }
}

fn update_y(binary_indexed_tree: &mut [Vec<i32>], x: usize, mut y: usize, delta: i32) {
    while y < binary_indexed_tree[x].len() {
        binary_indexed_tree[x][y] += delta;
        y += ((y as i32) & -(y as i32)) as usize;
    }
}
