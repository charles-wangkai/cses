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
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b));
}

fn solve(n: usize, a: &[usize], b: &[usize]) -> i32 {
    let mut adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
    }

    let mut dp = vec![vec![0; n]; 1 << n];
    dp[1][0] = 1;
    for mask in 1..1 << n {
        for (node, adj_vecs_node) in adj_vecs.iter().enumerate() {
            if dp[mask][node] != 0 {
                for &adj in adj_vecs_node {
                    if ((mask >> adj) & 1) == 0 {
                        let next_mask = mask | (1 << adj);
                        dp[next_mask][adj] = MOD_INT.add_mod(dp[next_mask][adj], dp[mask][node]);
                    }
                }
            }
        }
    }

    dp[(1 << n) - 1][n - 1]
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
