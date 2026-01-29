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
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, m));
}

fn solve(x: &[i32], m: i32) -> i32 {
    let mut dp = vec![vec![0; (m as usize) + 1]; x.len()];
    if x[0] == 0 {
        for i in 1..=m as usize {
            dp[0][i] = 1;
        }
    } else {
        dp[0][x[0] as usize] = 1;
    }

    for i in 1..x.len() {
        for j in 1..=m as usize {
            if x[i] == j as i32 || x[i] == 0 {
                for d in -1..=1i32 {
                    let prev = (j as i32) + d;
                    if (1..=m).contains(&prev) {
                        dp[i][j] = MOD_INT.add_mod(dp[i][j], dp[i - 1][prev as usize]);
                    }
                }
            }
        }
    }

    dp.last()
        .unwrap()
        .iter()
        .copied()
        .reduce(|acc, x| MOD_INT.add_mod(acc, x))
        .unwrap()
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
