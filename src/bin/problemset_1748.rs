use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead, BufReader},
};

const MOD_INT: ModInt = ModInt {
    modulus: 1_000_000_007,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x));
}

fn solve(x: &[i32]) -> i32 {
    let mut values = x
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .iter()
        .copied()
        .collect::<Vec<_>>();
    values.sort_unstable();

    let value_to_compressed = (0..values.len())
        .map(|i| (values[i], i + 1))
        .collect::<HashMap<_, _>>();

    let mut result = 0;
    let mut fenwick_tree = FenwickTree::new(value_to_compressed.len());
    for xi in x {
        let way_num = MOD_INT.add_mod(
            fenwick_tree.compute_prefix_sum(value_to_compressed[xi] - 1),
            1,
        );
        result = MOD_INT.add_mod(result, way_num);
        fenwick_tree.add(value_to_compressed[xi], way_num);
    }

    result
}

struct ModInt {
    modulus: i32,
}

#[allow(dead_code)]
impl ModInt {
    fn modulo(&self, x: i64) -> i32 {
        x.rem_euclid(self.modulus as i64) as i32
    }

    fn mod_inv(&self, x: i32) -> i32 {
        self.pow_mod(x, (self.modulus - 2) as i64)
    }

    fn add_mod(&self, x: i32, y: i32) -> i32 {
        self.modulo((x + y) as i64)
    }

    fn multiply_mod(&self, x: i32, y: i32) -> i32 {
        self.modulo((x as i64) * (y as i64))
    }

    fn divide_mod(&self, x: i32, y: i32) -> i32 {
        self.multiply_mod(x, self.mod_inv(y))
    }

    fn pow_mod(&self, base: i32, exponent: i64) -> i32 {
        if exponent == 0 {
            return 1;
        }

        self.multiply_mod(
            if exponent % 2 == 0 { 1 } else { base },
            self.pow_mod(self.multiply_mod(base, base), exponent / 2),
        )
    }
}

struct FenwickTree {
    a: Vec<i32>,
}

#[allow(dead_code)]
impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            a: vec![0; size + 1],
        }
    }

    fn add(&mut self, mut pos: usize, delta: i32) {
        while pos < self.a.len() {
            self.a[pos] = MOD_INT.add_mod(self.a[pos], delta);
            pos += ((pos as i32) & -(pos as i32)) as usize;
        }
    }

    fn compute_prefix_sum(&self, mut pos: usize) -> i32 {
        let mut result = 0;
        while pos != 0 {
            result = MOD_INT.add_mod(result, self.a[pos]);
            pos -= ((pos as i32) & -(pos as i32)) as usize;
        }

        result
    }
}
