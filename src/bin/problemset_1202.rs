use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    io::{stdin, BufRead, BufReader},
};

const MODULUS: i32 = 1_000_000_007;

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
                            add_mod(route_nums[b[edge] - 1], route_nums[node]);
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

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
