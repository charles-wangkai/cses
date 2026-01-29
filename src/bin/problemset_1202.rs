use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
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
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        c.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b, &c));
}

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32]) -> String {
    let mut edge_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        edge_vecs[a[i] - 1].push(i);
    }

    let mut prices = vec![i64::MAX; n];
    prices[0] = 0;
    let mut route_nums = vec![i32::MAX; n];
    route_nums[0] = 1;
    let mut min_flight_nums = vec![i32::MAX; n];
    min_flight_nums[0] = 0;
    let mut max_flight_nums = vec![i32::MAX; n];
    max_flight_nums[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((Reverse(price), node)) = heap.pop() {
        if price == prices[node] {
            for &edge in &edge_vecs[node] {
                let next_price = prices[node] + (c[edge] as i64);
                match next_price.cmp(&prices[b[edge] - 1]) {
                    Ordering::Less => {
                        prices[b[edge] - 1] = next_price;
                        route_nums[b[edge] - 1] = route_nums[node];
                        min_flight_nums[b[edge] - 1] = min_flight_nums[node] + 1;
                        max_flight_nums[b[edge] - 1] = max_flight_nums[node] + 1;

                        heap.push((Reverse(prices[b[edge] - 1]), b[edge] - 1));
                    }
                    Ordering::Equal => {
                        route_nums[b[edge] - 1] =
                            MOD_INT.add_mod(route_nums[b[edge] - 1], route_nums[node]);
                        min_flight_nums[b[edge] - 1] =
                            min_flight_nums[b[edge] - 1].min(min_flight_nums[node] + 1);
                        max_flight_nums[b[edge] - 1] =
                            max_flight_nums[b[edge] - 1].max(max_flight_nums[node] + 1);
                    }
                    Ordering::Greater => {}
                }
            }
        }
    }

    format!(
        "{} {} {} {}",
        prices[n - 1],
        route_nums[n - 1],
        min_flight_nums[n - 1],
        max_flight_nums[n - 1]
    )
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
