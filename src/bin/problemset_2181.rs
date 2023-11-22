use std::io::{stdin, BufRead, BufReader};

const MODULUS: i32 = 1_000_000_007;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n, m));
}

fn solve(n: usize, m: i32) -> i32 {
    let mut next_mask_vecs = vec![Vec::new(); 1 << n];
    for (curr_mask, next_mask_vec) in next_mask_vecs.iter_mut().enumerate() {
        for next_mask in 0..1 << n {
            if check(n, curr_mask, next_mask) {
                next_mask_vec.push(next_mask);
            }
        }
    }

    let mut dp = vec![0; 1 << n];
    dp[0] = 1;

    for _ in 0..m {
        let mut next_dp = vec![0; dp.len()];
        for curr_mask in 0..dp.len() {
            for &next_mask in &next_mask_vecs[curr_mask] {
                next_dp[next_mask] = add_mod(next_dp[next_mask], dp[curr_mask]);
            }
        }

        dp = next_dp;
    }

    dp[0]
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}

fn check(n: usize, curr_mask: usize, next_mask: usize) -> bool {
    let mut filled = Vec::new();
    for i in 0..n {
        if ((next_mask >> i) & 1) == 1 {
            if ((curr_mask >> i) & 1) == 1 {
                return false;
            }

            filled.push(true);
        } else {
            filled.push(((curr_mask >> i) & 1) == 1);
        }
    }

    let mut length = 0;
    for i in 0..=filled.len() {
        if i != filled.len() && !filled[i] {
            length += 1;
        } else {
            if length % 2 == 1 {
                return false;
            }

            length = 0;
        }
    }

    true
}
