use std::io::{stdin, BufRead, BufReader};

const OFFSETS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    split.next().unwrap();
    let mut squares = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let line: String = split.next().unwrap().parse().unwrap();
        squares.push(line.chars().collect());
    }

    println!("{}", solve(&mut squares));
}

fn solve(squares: &mut [Vec<char>]) -> i32 {
    let mut result = 0;
    for r in 0..squares.len() {
        for c in 0..squares[r].len() {
            if squares[r][c] == '.' {
                fill(squares, r, c);
                result += 1;
            }
        }
    }

    result
}

fn fill(squares: &mut [Vec<char>], r: usize, c: usize) {
    squares[r][c] = '#';

    for (r_offset, c_offset) in OFFSETS {
        let adj_r = (r as i32) + r_offset;
        let adj_c = (c as i32) + c_offset;
        if (0..squares.len() as i32).contains(&adj_r)
            && (0..squares[0].len() as i32).contains(&adj_c)
            && squares[adj_r as usize][adj_c as usize] == '.'
        {
            fill(squares, adj_r as usize, adj_c as usize);
        }
    }
}
