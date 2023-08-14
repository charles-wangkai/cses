use std::io::{stdin, BufRead, BufReader};

const MODULUS: i32 = 1_000_000_007;

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
                    dp[r][c] = add_mod(dp[r][c], dp[r - 1][c]);
                }
                if c != 0 {
                    dp[r][c] = add_mod(dp[r][c], dp[r][c - 1]);
                }
            }
        }
    }

    dp[n - 1][n - 1]
}

fn add_mod(x: i32, y: i32) -> i32 {
    (x + y).rem_euclid(MODULUS)
}
