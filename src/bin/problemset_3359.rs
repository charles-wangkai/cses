use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut grid = vec![vec!['\0'; n]; n];
    for r in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        for c in 0..n {
            grid[r][c] = line.as_bytes()[c] as char;
        }
    }

    println!("{}", solve(&grid));
}

fn solve(grid: &[Vec<char>]) -> String {
    let n = grid.len();

    let mut result = grid[0][0].to_string();
    let mut dp = vec![0];
    let mut seen = vec![false; n * n];
    for _ in 0..(2 * n - 2) {
        let mut next_level = Vec::new();
        for p in dp {
            let r = p / n;
            let c = p % n;
            if c != n - 1 {
                let next = r * n + (c + 1);
                if !seen[next] {
                    next_level.push(next);
                    seen[next] = true;
                }
            }
            if r != n - 1 {
                let next = (r + 1) * n + c;
                if !seen[next] {
                    next_level.push(next);
                    seen[next] = true;
                }
            }
        }

        let min_letter = next_level.iter().map(|p| grid[p / n][p % n]).min().unwrap();

        result.push(min_letter);
        dp = next_level
            .iter()
            .filter(|&p| grid[p / n][p % n] == min_letter)
            .copied()
            .collect::<Vec<_>>();
    }

    result
}
