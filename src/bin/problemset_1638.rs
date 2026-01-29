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
    let mut grid = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let line: String = split.next().unwrap().parse().unwrap();
        grid.push(line.chars().collect());
    }

    println!("{}", solve(&grid));
}

fn solve(grid: &[Vec<char>]) -> i32 {
    let n = grid.len();

    let mut dp = vec![vec![0; n]; n];
    for r in 0..n {
        for c in 0..n {
            if grid[r][c] == '.' {
                if r == 0 && c == 0 {
                    dp[r][c] = 1;
                }
                if r != 0 {
                    dp[r][c] = MOD_INT.add_mod(dp[r][c], dp[r - 1][c]);
                }
                if c != 0 {
                    dp[r][c] = MOD_INT.add_mod(dp[r][c], dp[r][c - 1]);
                }
            }
        }
    }

    dp[n - 1][n - 1]
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
