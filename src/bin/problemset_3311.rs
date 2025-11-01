use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut grid = vec![vec!['\0'; m]; n];
    for r in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let line: String = split.next().unwrap().parse().unwrap();
        let line: Vec<_> = line.chars().collect();
        for c in 0..m {
            grid[r][c] = line[c];
        }
    }

    println!("{}", solve(&grid));
}

fn solve(grid: &[Vec<char>]) -> String {
    let n = grid.len();
    let m = grid[0].len();

    let mut result = vec![vec!['\0'; m]; n];
    for r in 0..n {
        for c in 0..m {
            result[r][c] = ['A', 'B', 'C', 'D']
                .iter()
                .find(|&&x| {
                    x != grid[r][c]
                        && (r == 0 || x != result[r - 1][c])
                        && (c == 0 || x != result[r][c - 1])
                })
                .copied()
                .unwrap();
        }
    }

    result
        .iter()
        .map(|line| {
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
