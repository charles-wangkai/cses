use std::io::{stdin, BufRead, BufReader};

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
                next_dp[next_mask] = MOD_INT.add_mod(next_dp[next_mask], dp[curr_mask]);
            }
        }

        dp = next_dp;
    }

    dp[0]
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
